import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://squash-tf.readthedocs.io/en/latest/')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/h1_Welcome to Squash TF components documentation'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Squash TF Java Junit Runner'), 
    0)

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Squash TF Java Junit Runner'))

WebUI.click(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Squash TF Java Junit Runner'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/h1_Squash TF Java Junit Runner'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_Runner functions'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_Junit tests reference scheme'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_METADATA in JUnit Runner'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_TF Param Service'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_Creating projects'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Squash TF Java Junit Runner  Squash TF_5f4e23/a_METADATA in JUnit Runner'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Metadata in JUnit runner  Squash TF Ja_16dea5/h1_Metadata in JUnit runner'), 
    0)

WebUI.verifyElementText(findTestObject('Page_Metadata in JUnit runner  Squash TF Ja_16dea5/div_dependency  groupIdorg.squashtest.ta.ga_1d4d3e'), 
    'dependency')

WebUI.closeBrowser()

