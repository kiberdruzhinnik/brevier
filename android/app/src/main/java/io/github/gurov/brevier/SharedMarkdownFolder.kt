package io.github.gurov.brevier

import android.content.Context
import android.net.Uri
import android.provider.DocumentsContract
import java.io.ByteArrayOutputStream
import java.nio.charset.StandardCharsets
import java.security.MessageDigest

/** Android-only selection and preference state for the reader's shared Markdown cache. */
internal class SharedMarkdownFolder(context: Context) {
    private val resolver = context.contentResolver
    private val preferences = context.getSharedPreferences("shared-markdown-cache", Context.MODE_PRIVATE)

    val enabled: Boolean get() = preferences.getBoolean("enabled", false)
    val tree: Uri? get() = preferences.getString("tree", null)?.let(Uri::parse)
    val selected: Boolean get() = tree != null

    fun select(uri: Uri) {
        preferences.edit().putString("tree", uri.toString()).putBoolean("enabled", true).apply()
    }

    fun enable(on: Boolean) {
        preferences.edit().putBoolean("enabled", on && selected).apply()
    }

    /** Read a Markdown cache record by its fragment-free article address. */
    fun read(address: String): String? = runCatching {
        val uri = tree ?: return null
        val document = find(uri, fileName(address)) ?: return null
        resolver.openInputStream(document)?.use { input ->
            val output = ByteArrayOutputStream()
            val buffer = ByteArray(16 * 1024)
            var total = 0
            while (true) {
                val count = input.read(buffer)
                if (count < 0) break
                total += count
                if (total > MAX_RECORD_BYTES) return null
                output.write(buffer, 0, count)
            }
            StandardCharsets.UTF_8.newDecoder().decode(java.nio.ByteBuffer.wrap(output.toByteArray())).toString()
        }
    }.getOrNull()

    /** Replace the one current Markdown copy for this address, or create it. */
    fun write(address: String, markdown: String): Boolean = runCatching {
        val uri = tree ?: return false
        val name = fileName(address)
        val destination = find(uri, name) ?: run {
            val parentId = DocumentsContract.getTreeDocumentId(uri)
            val parent = DocumentsContract.buildDocumentUriUsingTree(uri, parentId)
            DocumentsContract.createDocument(resolver, parent, "text/markdown", name)
        } ?: return false
        val bytes = markdown.toByteArray(StandardCharsets.UTF_8)
        if (bytes.size > MAX_RECORD_BYTES) return false
        resolver.openOutputStream(destination, "wt")?.use { it.write(bytes) } ?: return false
        true
    }.getOrDefault(false)

    private fun find(uri: Uri, name: String): Uri? {
        val rootId = DocumentsContract.getTreeDocumentId(uri)
        val children = DocumentsContract.buildChildDocumentsUriUsingTree(uri, rootId)
        resolver.query(
            children,
            arrayOf(DocumentsContract.Document.COLUMN_DOCUMENT_ID, DocumentsContract.Document.COLUMN_DISPLAY_NAME),
            null,
            null,
            null,
        )?.use { cursor ->
            val idColumn = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_DOCUMENT_ID)
            val nameColumn = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_DISPLAY_NAME)
            while (cursor.moveToNext()) {
                if (cursor.getString(nameColumn) == name) {
                    return DocumentsContract.buildDocumentUriUsingTree(uri, cursor.getString(idColumn))
                }
            }
        }
        return null
    }

    private fun fileName(address: String): String {
        val identity = address.substringBefore('#')
        val hash = MessageDigest.getInstance("SHA-256")
            .digest(identity.toByteArray(StandardCharsets.UTF_8))
            .joinToString("") { "%02x".format(it) }
        return "$hash.md"
    }

    private companion object {
        const val MAX_RECORD_BYTES = 8 * 1024 * 1024
    }
}
