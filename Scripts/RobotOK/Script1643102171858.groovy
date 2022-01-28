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

WebUI.click(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/div_Docs               Welcome to Squash TF_84596f'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/h1_Welcome to Squash TF components documentation'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Runners'), 
    0)

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Runners'))

WebUI.click(findTestObject('Object Repository/Page_Welcome to Squash TF components docume_c5d9e5/a_Runners'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Squash TF Runners  Squash TF component_4bce69/h1_Squash TF Runners'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Runners  Squash TF component_4bce69/a_Robot Framework Runner'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Squash TF Runners  Squash TF component_4bce69/div_Docs               Squash TF Runners   _0e9efa'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Runners  Squash TF component_4bce69/a_Robot Framework Runner'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Squash TF Runners  Squash TF component_4bce69/a_Robot Framework Runner'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Squash TF Robot Framework Runner  Squa_e6b447/h1_Squash TF Robot Framework Runner'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Robot Framework Runner  Squa_e6b447/a_Runner functions'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Robot Framework Runner  Squa_e6b447/a_Metadata in Robot Framework Runner'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Squash TF Robot Framework Runner  Squa_e6b447/a_TF Param Service'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Squash TF Robot Framework Runner  Squa_e6b447/a_TF Param Service'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_TF Param Service  Squash TF Robot Fram_898208/h1_TF Param Service'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Page_TF Param Service  Squash TF Robot Fram_898208/div_python -m pip install squash-tf-services'), 
    'python -m pip install squash-tf-services')

WebUI.closeBrowser()

