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

WebUI.navigateToUrl('https://skf.readthedocs.io/projects/skf/en/doc-stable/index.html')

WebUI.waitForElementVisible(findTestObject('Object Repository/Page_Squash Keyword Framework  Squash TF Ke_3b6aff/h1_Squash Keyword Framework (1)'), 
    0)

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Squash Keyword Framework  Squash TF Ke_3b6aff/a_Libraries'))

WebUI.click(findTestObject('Object Repository/Page_Squash Keyword Framework  Squash TF Ke_3b6aff/a_Libraries'))

check = WebUI.getText(findTestObject('Page_SKF Plugins  Squash TF Keyword Framewo_587662/ul_Commons component PluginRepositoriesTarg_da6b5a'))

WebUI.verifyMatch(check, pluginName, true, FailureHandling.STOP_ON_FAILURE)

