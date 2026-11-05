package io.github.gurov.brevier

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class PdfTextVersionsTest {
    @Test
    fun regularTextIsInitialAndRecognizedTextCanBeReplacedByIt() {
        val versions = PdfTextVersions<String>()
        versions.openedRegular("native text")

        assertEquals("native text", versions.current)
        versions.openedRecognized("recognized text")
        assertEquals("recognized text", versions.current)
        assertEquals("native text", versions.restoreRegular())
        assertEquals("native text", versions.current)
        assertNull(versions.recognized)
    }

    @Test
    fun failedRecognitionLeavesRegularTextAvailable() {
        val versions = PdfTextVersions<String>()
        versions.openedRegular("native text")
        // A failed request does not call openedRecognized.

        assertEquals("native text", versions.current)
        assertEquals("native text", versions.restoreRegular())
    }

    @Test
    fun stageLabelsCoverEachLoadStep() {
        assertEquals("Network fetching", LoadingStage.NETWORK_FETCHING)
        assertEquals("Regular text extraction", LoadingStage.REGULAR_EXTRACTION)
        assertEquals("OCR text extraction", LoadingStage.OCR_EXTRACTION)
    }

    @Test
    fun pdfActionOffersRecognitionAndAPathBackToRegularText() {
        assertEquals("Recognize PDF text (OCR)", pdfTextActionLabel(false))
        assertEquals("Use regular PDF text", pdfTextActionLabel(true))
        assertEquals("Recognize PDF text (OCR)", pdfTextMenuActionLabel(isPdf = true, recognized = false))
        assertEquals("Use regular PDF text", pdfTextMenuActionLabel(isPdf = true, recognized = true))
        assertNull(pdfTextMenuActionLabel(isPdf = false, recognized = false))
        assertNull(pdfTextMenuActionLabel(isPdf = false, recognized = true))
    }
}
