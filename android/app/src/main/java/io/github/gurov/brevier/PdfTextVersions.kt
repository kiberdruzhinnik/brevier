package io.github.gurov.brevier

/** Keeps the regular result while an optional OCR result is being viewed. */
class PdfTextVersions<T> {
    var regular: T? = null
        private set
    var recognized: T? = null
        private set

    val current: T?
        get() = recognized ?: regular

    fun openedRegular(value: T) {
        regular = value
        recognized = null
    }

    fun openedRecognized(value: T) {
        recognized = value
    }

    fun restoreRegular(): T? {
        recognized = null
        return regular
    }

    fun clear() {
        regular = null
        recognized = null
    }
}

object LoadingStage {
    const val NETWORK_FETCHING = "Network fetching"
    const val REGULAR_EXTRACTION = "Regular text extraction"
    const val OCR_EXTRACTION = "OCR text extraction"
}

fun pdfTextActionLabel(recognized: Boolean): String =
    if (recognized) "Use regular PDF text" else "Recognize PDF text (OCR)"

fun pdfTextMenuActionLabel(isPdf: Boolean, recognized: Boolean): String? =
    if (isPdf) pdfTextActionLabel(recognized) else null
