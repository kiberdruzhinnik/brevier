import java.util.Properties
import java.net.URL
import java.security.MessageDigest
import java.util.zip.ZipFile

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

// Корень репозитория: ядро, его Cargo.lock и гарнитуры лежат там, а не здесь.
val repo: File = rootProject.projectDir.parentFile

// Версия приложения — версия ядра: у Brevier одна версия на все платформы.
val coreVersion: String = Regex("""(?m)^version\s*=\s*"([^"]+)"""")
    .find(repo.resolve("Cargo.toml").readText())
    ?.groupValues?.get(1)
    ?: error("no version in Cargo.toml")

// Номер версии для Android выводится из неё же (0.5.2 → 502), но записан
// числом: Gradle сверяет его с версией ниже.
val derivedVersionCode: Int = coreVersion.split('.').fold(0) { code, part -> code * 100 + part.toInt() }

// Выпуск для телефона содержит только arm64; отладка сохраняет ABI эмулятора.
// Один и тот же список управляет сборкой ядра и содержимым APK.
val requestedTasks = gradle.startParameter.taskNames.map { it.substringAfterLast(':') }
val releaseRequested = requestedTasks.any {
    it.contains("release", ignoreCase = true) || it.equals("assemble", ignoreCase = true) || it.equals("build", ignoreCase = true)
}
val debugRequested = requestedTasks.any { it.contains("debug", ignoreCase = true) }
val selectedAbis = (findProperty("abis") as String?)
    ?.split(',')?.map { it.trim() }?.filter { it.isNotEmpty() }
val releaseAbis = listOf("arm64-v8a")
if (releaseRequested && selectedAbis != null && selectedAbis != releaseAbis) {
    error("Android release APKs support arm64-v8a only; remove -Pabis or use -Pabis=arm64-v8a")
}
val abis: List<String> = selectedAbis
    ?: if (releaseRequested || !debugRequested) releaseAbis else listOf("arm64-v8a", "x86_64")

val coreDir = layout.buildDirectory.dir("core")
val pdfiumDir = layout.buildDirectory.dir("pdfium")

// PDFium is loaded dynamically by pdfium-render. Keep the upstream binary
// pinned by release and digest, with one shared jniLibs folder per ABI.
val preparePdfium by tasks.registering {
    val selected = abis.toList()
    inputs.property("abis", selected)
    outputs.dir(pdfiumDir)
    doLast {
        val release = "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F8125"
        val artifacts = mapOf(
            "arm64-v8a" to ("pdfium-android-arm64.tgz" to "0495e57163429937caaee1bdd9b07924a7098dbd51d1b97645d124e349f097b4"),
            "x86_64" to ("pdfium-android-x64.tgz" to "bd68707f29f2316c20f9d5c793b8731796a4a65408ac4d3c30bc606390354e41"),
        )
        val destination = pdfiumDir.get().asFile
        selected.forEach { abi ->
            val (archiveName, expectedDigest) = artifacts[abi] ?: error("No pinned PDFium binary for $abi")
            val library = destination.resolve("jniLibs/$abi/libpdfium.so")
            val licenseDir = destination.resolve("assets/pdfium")
            if (!library.isFile || !licenseDir.resolve("LICENSE").isFile) {
                val archive = temporaryDir.resolve(archiveName)
                URL("$release/$archiveName").openStream().use { input ->
                    archive.outputStream().use { output -> input.copyTo(output) }
                }
                val digest = MessageDigest.getInstance("SHA-256").digest(archive.readBytes())
                    .joinToString("") { "%02x".format(it) }
                check(digest == expectedDigest) { "PDFium archive digest mismatch for $abi" }
                if (!library.isFile) {
                    library.parentFile.mkdirs()
                    val result = ProcessBuilder("tar", "-xzf", archive.absolutePath, "-C", library.parentFile.absolutePath, "lib/libpdfium.so")
                        .redirectErrorStream(true).start()
                    val output = result.inputStream.bufferedReader().readText()
                    check(result.waitFor() == 0) { "Could not unpack PDFium for $abi: $output" }
                    val extractedLibrary = library.parentFile.resolve("lib/libpdfium.so")
                    check(extractedLibrary.isFile) { "PDFium archive did not contain lib/libpdfium.so for $abi" }
                    extractedLibrary.copyTo(library, overwrite = true)
                    extractedLibrary.parentFile.deleteRecursively()
                }
                if (!licenseDir.resolve("LICENSE").isFile) {
                    licenseDir.mkdirs()
                    val licenses = ProcessBuilder("tar", "-xzf", archive.absolutePath, "-C", licenseDir.absolutePath, "LICENSE", "licenses")
                        .redirectErrorStream(true).start()
                    val licenseOutput = licenses.inputStream.bufferedReader().readText()
                    check(licenses.waitFor() == 0) { "Could not unpack PDFium licenses: $licenseOutput" }
                }
            }
        }
    }
}

// Подпись выпуска. Ключ в репозиторий не едет: путь к файлу свойств
// (`storeFile`, `storePassword`, `keyAlias`) называет переменная
// BREVIER_SIGNING — на машине мейнтейнера это ~/Android/keys, в CI файл
// собирается из секретов. Без неё выпуск не собирается вовсе: неподписанный
// APK не поставить, а подписанный отладочным ключом раздавать нельзя.
val signing: Properties? = System.getenv("BREVIER_SIGNING")
    ?.takeIf { it.isNotBlank() }
    ?.let { path -> Properties().apply { file(path).inputStream().use { load(it) } } }

// Неподписанный выпуск нужен для проверки воспроизводимости в CI.
val unsigned: Boolean = (findProperty("unsigned") as String?)?.let { it != "false" } ?: false

// Имя файла — то, что человек скачает: brevier-0.2.0-arm64.apk, а не
// app-release.apk. ABI в имени, когда она одна; отладочная сборка помечена.
fun apkName(buildType: String): String {
    val abi = when (abis) {
        listOf("arm64-v8a") -> "-arm64"
        listOf("x86_64") -> "-x86_64"
        else -> ""
    }
    val kind = when {
        buildType == "debug" -> "-debug"
        unsigned -> "-unsigned"
        else -> ""
    }
    return "brevier-$coreVersion$abi$kind.apk"
}

// Ядро собирает cargo, а не Gradle: это та же команда, что у канарейки в CI.
// NDK — тот, что назван `ndkVersion` ниже, а не первый попавшийся: на раннере
// GitHub ANDROID_NDK_HOME смотрит в свой, а у мейнтейнера может быть другой;
// библиотека выходила бы разной.
val buildCore by tasks.registering(Exec::class) {
    description = "Builds the Brevier core as a shared library for each ABI"
    workingDir = repo
    environment("ANDROID_NDK_HOME", android.ndkDirectory.path)
    commandLine(listOf("sh", "android/core.sh", coreDir.get().asFile.path) + abis)
    inputs.dir(repo.resolve("src"))
    inputs.dir(repo.resolve("assets/hyph"))
    inputs.file(repo.resolve("Cargo.toml"))
    inputs.file(repo.resolve("Cargo.lock"))
    inputs.file(repo.resolve("android/core.sh"))
    inputs.file(repo.resolve("android/rust-toolchain.toml"))
    inputs.property("abis", abis)
    inputs.property("ndk", android.ndkVersion)
    outputs.dir(coreDir)
}

android {
    namespace = "io.github.gurov.brevier"
    compileSdk = 36

    defaultConfig {
        applicationId = "io.github.gurov.brevier"
        // API 24 — та же нижняя граница, что у канарейки в CI: ниже системный
        // проверяющий сертификаты не знает отзыва.
        minSdk = 24
        targetSdk = 36
        versionName = coreVersion
        versionCode = 2200
        check(versionCode == derivedVersionCode) {
            "versionCode is $versionCode, but version $coreVersion means $derivedVersionCode: update it in android/app/build.gradle.kts"
        }
        ndk { abiFilters += abis }
    }

    // Тот же NDK в CI и на машине мейнтейнера: иначе ядро собирается
    // разными компиляторами, и воспроизводимой сборки нет.
    ndkVersion = "29.0.14206865"

    // `core.sh` уже стрипает ядро тем же NDK; второй проход AGP лишний.
    packaging { jniLibs { keepDebugSymbols += "**/libbrevier.so" } }

    sourceSets["main"].apply {
        // Гарнитуры и их лицензия входят в Android-приложение вместе.
        assets.srcDir(repo.resolve("assets/fonts"))
        assets.srcDir(repo.resolve("assets/licenses"))
        assets.srcDir(pdfiumDir.map { it.dir("assets") })
        jniLibs.srcDir(coreDir)
        jniLibs.srcDir(pdfiumDir.map { it.dir("jniLibs") })
    }

    signingConfigs {
        if (signing != null) {
            create("release") {
                storeFile = file(signing.getProperty("storeFile"))
                storePassword = signing.getProperty("storePassword")
                keyAlias = signing.getProperty("keyAlias", "brevier")
                // PKCS12 держит один пароль на хранилище и ключ.
                keyPassword = signing.getProperty("keyPassword", storePassword)
            }
        }
    }

    buildTypes {
        release {
            // R8 выбрасывает неиспользуемый код. Всё, что зовут по JNI, он
            // не видит, — правила в proguard-rules.pro
            // и у модуля rustls-platform-verifier.
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = if (unsigned) null else signingConfigs.findByName("release")
            // Корень Gradle — `android/`, а git — уровнем выше: AGP писал
            // в APK «NO_SUPPORTED_VCS_FOUND», то есть ничего полезного.
            vcsInfo { include = false }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions { jvmTarget = "17" }
}

tasks.named("preBuild") { dependsOn(buildCore, preparePdfium) }

val verifyReleaseCxxRuntime by tasks.registering {
    val releaseApk = layout.buildDirectory.file("outputs/apk/release/${apkName("release")}")
    dependsOn("packageRelease")
    doLast {
        ZipFile(releaseApk.get().asFile).use { apk ->
            abis.forEach { abi ->
                check(apk.getEntry("lib/$abi/libc++_shared.so") != null) {
                    "Release APK is missing libc++_shared.so for $abi"
                }
            }
        }
    }
}

tasks.configureEach {
    if (name == "assembleRelease") dependsOn(verifyReleaseCxxRuntime)
}

@Suppress("DEPRECATION")
android.applicationVariants.all {
    val type = buildType.name
    outputs.all {
        (this as com.android.build.gradle.internal.api.BaseVariantOutputImpl).outputFileName = apkName(type)
    }
}

// Выпуск без ключа — сразу и словами, а не неподписанным файлом в конце.
gradle.taskGraph.whenReady {
    if (allTasks.any { it.project == project && it.name.contains("Release") } && abis != releaseAbis) {
        throw GradleException("Android release APKs support arm64-v8a only; use -Pabis=arm64-v8a")
    }
    if (signing == null && !unsigned && allTasks.any { it.project == project && it.name.contains("Release") && it.name.startsWith("assemble") }) {
        throw GradleException("A release is signed: set BREVIER_SIGNING to a properties file with storeFile, storePassword and keyAlias (or pass -Punsigned for an unsigned build)")
    }
}

dependencies {
    // Версию с Cargo.lock сверяет сам модуль.
    implementation(project(":rustls-platform-verifier"))
    testImplementation("junit:junit:4.13.2")
}
