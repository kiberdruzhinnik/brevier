package io.github.gurov.brevier

import android.content.Context
import android.graphics.Color
import android.graphics.Typeface
import android.util.TypedValue
import org.json.JSONObject

/**
 * Типографская модель ядра (`outline.rs`, `palette.rs`). Kotlin чисел
 * не повторяет: кегли, мера, отступы и цвета приходят из ядра одним ответом.
 */
class Typography(json: JSONObject) {
    val textSize = json.getDouble("textSize").toFloat()
    /** Кегль десктопа в его пикселях: от него считаются все отступы модели. */
    val textPx = json.getDouble("textPx").toFloat()
    val lineHeight = json.getDouble("lineHeight").toFloat()
    val headings = floats(json, "headings")
    val headingWeights = json.getJSONArray("headingWeights").map { (it as Number).toInt() }
    val measureInEms = json.getDouble("measureInEms").toFloat()
    val zoomSteps = floats(json, "zoomSteps")
    val zoomNormal = json.getInt("zoomNormal")
    val indent = json.getDouble("indent").toFloat()
    val hang = json.getDouble("hang").toFloat()
    /** Висячий отступ пункта с номером в 1, 2, 3 цифры (#13). */
    val numberHangs = floats(json, "numberHangs")
    val noteIndent = json.getDouble("noteIndent").toFloat()
    val noteHang = json.getDouble("noteHang").toFloat()
    val codeGap = json.getDouble("codeGap").toFloat()
    val codeSize = json.getDouble("codeSize").toFloat()
    val padSize = json.getDouble("padSize").toFloat()
    val noteSize = json.getDouble("noteSize").toFloat()
    val noterefSize = json.getDouble("noterefSize").toFloat()
    val noterefRise = json.getDouble("noterefRise").toFloat()
    val alertSize = json.getDouble("alertSize").toFloat()
    val alertTracking = json.getDouble("alertTracking").toFloat()
    val ruleWidth = json.getDouble("ruleWidth").toFloat()
    val ruleX = json.getDouble("ruleX").toFloat()
    val ruleInset = json.getDouble("ruleInset").toFloat()
    val quoteLevels = json.getInt("quoteLevels")
    val found = color(json.getString("found"))
    val foundHere = color(json.getString("foundHere"))
    val foundInk = color(json.getString("foundInk"))
    val light = Palette(json.getJSONObject("light"))
    val dark = Palette(json.getJSONObject("dark"))
    /** Высокий контраст (#44): белая бумага и чёрная краска, в тёмной — наоборот. */
    val lightHigh = Palette(json.getJSONObject("lightHigh"))
    val darkHigh = Palette(json.getJSONObject("darkHigh"))

    fun palette(theme: Theme): Palette = when {
        theme.dark && theme.high -> darkHigh
        theme.dark -> darkPaper(dark, theme.darkPaper)
        theme.high -> lightHigh
        else -> lightPaper(light, theme.paper)
    }

    private fun lightPaper(base: Palette, tone: String) = when (tone) {
        "white" -> base.copy(paper = color("#ffffff"), shelf = color("#f2f2f2"), panel = color("#f0f0f0"), rule = color("#d4d4d4"), chosen = color("#dedede"), touched = color("#e9e9e9"))
        "grey" -> base.copy(paper = color("#e6e6e6"), shelf = color("#dcdcdc"), dim = color("#625c53"), comment = color("#625c53"), panel = color("#dedede"), rule = color("#c5c5c5"), chosen = color("#cecece"), touched = color("#d6d6d6"))
        "sepia" -> base.copy(paper = color("#f1e3c9"), ink = color("#2c2118"), shelf = color("#e8d9bd"), dim = color("#6a5c4d"), comment = color("#6a5c4d"), panel = color("#eadcc1"), rule = color("#d8c8aa"), chosen = color("#ddc9a8"), touched = color("#e5d4b7"))
        else -> base
    }

    private fun darkPaper(base: Palette, tone: String) = when (tone) {
        "grey" -> base.copy(paper = color("#2c2c2c"), ink = color("#e2e2e2"), shelf = color("#222222"), dim = color("#b5b5b5"), comment = color("#b5b5b5"), panel = color("#353535"), rule = color("#555555"), chosen = color("#414141"), touched = color("#303030"))
        "black" -> base.copy(paper = color("#000000"), ink = color("#ffffff"), shelf = color("#111111"), link = color("#9fd2ff"), dim = color("#c0c0c0"), comment = color("#c0c0c0"), panel = color("#1a1a1a"), rule = color("#707070"), chosen = color("#303030"), touched = color("#242424"))
        else -> base
    }

    /** Висячий отступ по ширине маркера: 0 — «•», 1..3 — цифр в номере (`outline::hang`). */
    fun hang(width: Int) = if (width == 0) hang else numberHangs[minOf(width, numberHangs.size) - 1]

    companion object {
        private fun floats(json: JSONObject, key: String) =
            json.getJSONArray(key).map { (it as Number).toFloat() }

        fun color(hex: String): Int = Color.parseColor(hex)

        /** Одна на процесс: модель неизменна, спрашивать ядро дважды незачем. */
        val shared: Typography by lazy { Typography(Core.typography()) }
    }
}

/** Чем печатать: тема и контраст — как `palette::Theme` в ядре. */
data class Theme(val dark: Boolean, val high: Boolean, val paper: String = "ivory", val darkPaper: String = "warm")

/** Цвета темы. */
data class Palette(
    val paper: Int,
    val ink: Int,
    val shelf: Int,
    val link: Int,
    val dim: Int,
    val panel: Int,
    val keyword: Int,
    val literal: Int,
    val number: Int,
    val comment: Int,
    val rule: Int,
    val chosen: Int,
    val touched: Int,
) {
    constructor(json: JSONObject) : this(
        paper = Typography.color(json.getString("paper")),
        ink = Typography.color(json.getString("ink")),
        shelf = Typography.color(json.getString("shelf")),
        link = Typography.color(json.getString("link")),
        dim = Typography.color(json.getString("dim")),
        panel = Typography.color(json.getString("panel")),
        keyword = Typography.color(json.getString("keyword")),
        literal = Typography.color(json.getString("literal")),
        number = Typography.color(json.getString("number")),
        comment = Typography.color(json.getString("comment")),
        rule = Typography.color(json.getString("rule")),
        chosen = Typography.color(json.getString("chosen")),
        touched = Typography.color(json.getString("touched")),
    )
    /** Линейка цитаты: приглушённая краска вполсилы — как в окне. */
    val quoteRule: Int get() = Color.argb(128, Color.red(dim), Color.green(dim), Color.blue(dim))
}

/**
 * Набор читателя (#4): гарнитура, кегль, мера в кеглях, интерлиньяж и выключка.
 * Числа и их подписи — из ядра (`outline::Type`), здесь только носим.
 */
data class ReaderType(
    val face: String,
    val size: Float,
    val measure: Float,
    val leading: Float,
    val justify: Boolean,
    val paragraphs: Float,
) {
    companion object {
        fun of(json: JSONObject) = ReaderType(
            face = json.optString("face", "noto"),
            size = json.getDouble("size").toFloat(),
            measure = json.getDouble("measure").toFloat(),
            leading = json.getDouble("leading").toFloat(),
            justify = json.optBoolean("justify"),
            paragraphs = json.optDouble("paragraphs", 0.5).toFloat(),
        )
    }
}

/**
 * Модель в пикселях этого экрана на этой ступени масштаба.
 *
 * Кегль берём в `sp`: 16.5 sp на телефоне читаются так же, как 16.5 пункта
 * на мониторе в полутора метрах дальше, — и системная настройка размера
 * шрифта при этом уважается. Всё остальное, что в модели задано пикселями
 * десктопа, умножается на отношение нашего кегля к десктопному: пропорции
 * страницы одни на обеих платформах.
 */
class Metrics(context: Context, val type: Typography, val zoom: Float, val reader: ReaderType) {
    /** Кегль читателя на ступени масштаба: он множит всю модель, как и ступень. */
    val text: Float = TypedValue.applyDimension(
        TypedValue.COMPLEX_UNIT_SP, reader.size * zoom, context.resources.displayMetrics,
    )
    /** Сколько наших пикселей в пикселе десктопа. */
    val unit: Float = text / type.textPx
    /** Воздух между строками и абзацами — та же величина, что `extra` в окне. */
    val extra: Float = (reader.leading - 1f) * type.textSize * unit
    val gap: Float = reader.paragraphs * text * reader.leading
    val indent: Float = if (reader.paragraphs < 0.001f) text else 0f
    /** Мера: колонка не шире этого. На телефоне она обычно — весь экран. */
    val measure: Float = reader.measure * text
    /** Гарнитура текста: `noto`, `serif` или `sans`. */
    val face: String get() = reader.face
    /** Выключка по ширине (#6). */
    val justify: Boolean get() = reader.justify

    fun px(desktop: Float): Float = desktop * unit
}

/**
 * Гарнитуры из комплекта: те же файлы Noto, что вшиты в десктоп. Шрифт
 * выбирает не система — иначе обещание «типографику задаёт читатель, а не
 * платформа» не выполнялось бы и здесь.
 */
class Fonts(context: Context) {
    private val assets = context.assets
    private fun load(name: String): Typeface = Typeface.createFromAsset(assets, name)

    val light = load("NotoSans-Light.ttf")
    val regular = load("NotoSans-Regular.ttf")
    val italic = load("NotoSans-Italic.ttf")
    val medium = load("NotoSans-Medium.ttf")
    val bold = load("NotoSans-Bold.ttf")
    val boldItalic = load("NotoSans-BoldItalic.ttf")
    val mono = load("NotoSansMono-Regular.ttf")

    /**
     * Начертание под вес и наклон. Курсив есть только у 400 и 700; лёгкому
     * и среднему курсиву достаётся прямое начертание с наклоном — ровно
     * то, что делает Pango, когда нужного файла в комплекте нет.
     */
    fun pick(mono: Boolean, weight: Int, italic: Boolean, face: String = "noto"): Face {
        if (mono) return Face(this.mono, skew = italic, fakeBold = weight >= 600)
        if (face != "noto") return system(face, weight, italic)
        return when {
            weight >= 600 -> if (italic) Face(boldItalic) else Face(bold)
            weight >= 500 -> Face(medium, skew = italic)
            weight >= 400 -> if (italic) Face(this.italic) else Face(regular)
            else -> Face(light, skew = italic)
        }
    }

    /**
     * Гарнитура системы (#4): антиква или гротеск, которые Android называет
     * `serif` и `sans-serif`. С Android 9 вес любой; раньше — только обычный
     * и жирный, а лёгкий заголовок встаёт обычным.
     */
    private fun system(face: String, weight: Int, italic: Boolean): Face {
        val family = if (face == "serif") Typeface.SERIF else Typeface.SANS_SERIF
        val key = Triple(family, weight, italic)
        val typeface = systemFaces.getOrPut(key) {
            if (android.os.Build.VERSION.SDK_INT >= 28) {
                Typeface.create(family, weight, italic)
            } else {
                val style = when {
                    weight >= 600 && italic -> Typeface.BOLD_ITALIC
                    weight >= 600 -> Typeface.BOLD
                    italic -> Typeface.ITALIC
                    else -> Typeface.NORMAL
                }
                Typeface.create(family, style)
            }
        }
        return Face(typeface)
    }

    private val systemFaces = HashMap<Triple<Typeface, Int, Boolean>, Typeface>()

    class Face(val typeface: Typeface, val skew: Boolean = false, val fakeBold: Boolean = false)

    companion object {
        @Volatile
        private var loaded: Fonts? = null

        fun of(context: Context): Fonts =
            loaded ?: synchronized(this) { loaded ?: Fonts(context.applicationContext).also { loaded = it } }
    }
}
