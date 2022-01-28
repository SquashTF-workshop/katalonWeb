<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_TF Param ServiceThis component provided_f14eff</name>
   <tag></tag>
   <elementGuidId>3c260411-44c9-410c-9a83-42a5643dad58</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='tf-param-service']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#tf-param-service</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>section</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>tf-param-service</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
TF Param Service¶
This component provided by the squash-tf-services library allows you to retrieve the values
of the parameters transmitted to the runner through the json test suite file
(typically provided by Squash TM) in order to use them in your robotframework tests.
All keywords defined by the library allow the caller to define default values in the test,
so that it may be run outside of the runner.

Configuration¶

In order to use the TFParamService keywords in your test suite, you have to

install the python library. We assume here that the pip python package manager is installed.
python -m pip install squash-tf-services


Make sure that this command has been executed in all environement where the tests will be run,
in the relevant python environement. If you use a symlink and path setting in a linux environment
to use python3 as your ‘python’ binary, make sure this setting is in effect when you run the install
command.

Import the keyword library by using the following Library statement in your test :
Library squash_tf.TFParamService








Using parameter values in your tests¶
To retrieve parameter values in your tests, use one of the three defined keywords in the following way :
${value}= Get Param    &lt;key>    [&lt;optional_default_value>]


You may omit the optional default value if None is manageable in your test.
Otherwise, the second parameter will define a default value used if no parameter value is
defined for your key. This will happen if :

The test is executed outside of the runner
There is no parameter value for the key you have used
(for exemple, no value for this test case)


Known limitations¶
If you use python2, encoding will not be properly managed in parameter values.
This problem does not exist if you run the same tests using python3. As python2 will reached
its end of life on next january (official EOL date 2020/01/01), we recommend using python3.



Keywords¶










${value}=
Get Global Param
key
 

${value}=
Get Global Param
key
default_value



This keyword retrieves the value of a parameter defined for all tests executed in this run.
If there is a global value mapped on this key, the value argument is assigned with it,
otherwise it is assigned with the default value. If no default value was given, value
becomes None.
Ex :
${parameter}=    Get Global Param    target_base_url    http:localhost:8080/sut/











${value}=
Get Test Param
key
 

${value}=
Get Test Param
key
default_value



This keyword retrieves the value of a parameter defined for one execution of a test case
in this run.
This means that the run, as built through the json test suite file build by Squash TM,
may schedule several execution of the same test case with different value for the parameter.
If there is a value mapped on this key for the specific test case run,
the value argument is assigned with it,otherwise it is assigned with the default value.
If no default value was given, value becomes None.
Ex :
${login}=    Get Test Param    TC_login    test_user











${value}=
Get Param
key
 

${value}=
Get Param
key
default_value



This keyword combines both scopes. If a parameter value is mapped on this key
for this test case run, then the value argument is assigned with this value.
If not, global parameters are queried. If a global parameter value is mapped on the given key,
then the value argument is assigned with this value. Otherwise, value becomes None.
Ex :
${login}=    Get Param    login    test_user







Manually providing a .json file¶
If you want to manually provide a .json file, you need to add the following parameter on your runner command line
-Dtf.test.suite={file:path/to/json/FileName.json}.
“path/to/json/FileName.json” must be the relative path of your .json file from the root of your project.
If the .json file is located directly at the root of your project, just type -Dtf.test.suite={file:FileName.json}
For example :
mvn org.squashtest.ta.galaxia:squash-tf-robotframework-runner-maven-plugin:1.0.0-RELEASE:run -Dtf.test.suite={file:testSuite.json}


{
    &quot;test&quot;: [{
            &quot;id&quot;: &quot;39&quot;,
            &quot;script&quot;: &quot;My Test Suite/First Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;TC_UUID&quot;: &quot;3a7099ff-ab59-4e99-b21d-07e7d71d1ed5&quot;
            }
        }, {
            &quot;id&quot;: &quot;40&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Bertrand&quot;,
                &quot;DSNAME&quot;: &quot;dataset1&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;41&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-d836370d519b&quot;
            }
        }, {
            &quot;id&quot;: &quot;41&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Damien&quot;,
                &quot;DSNAME&quot;: &quot;dataset2&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;undefined&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-a036bf0d519b&quot;
            }
        }],
    &quot;param&quot;: {
        &quot;globalParamSection&quot;: &quot;This is global param section&quot;,
        &quot;user&quot;: &quot;foo&quot;,
        &quot;ow_ner.Na-me&quot;: &quot;bar&quot;,
    }
}



</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;tf-param-service&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='tf-param-service']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Edit on Bitbucket'])[1]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='TF Param Service'])[2]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[2]/div/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'tf-param-service' and (text() = '
TF Param Service¶
This component provided by the squash-tf-services library allows you to retrieve the values
of the parameters transmitted to the runner through the json test suite file
(typically provided by Squash TM) in order to use them in your robotframework tests.
All keywords defined by the library allow the caller to define default values in the test,
so that it may be run outside of the runner.

Configuration¶

In order to use the TFParamService keywords in your test suite, you have to

install the python library. We assume here that the pip python package manager is installed.
python -m pip install squash-tf-services


Make sure that this command has been executed in all environement where the tests will be run,
in the relevant python environement. If you use a symlink and path setting in a linux environment
to use python3 as your ‘python’ binary, make sure this setting is in effect when you run the install
command.

Import the keyword library by using the following Library statement in your test :
Library squash_tf.TFParamService








Using parameter values in your tests¶
To retrieve parameter values in your tests, use one of the three defined keywords in the following way :
${value}= Get Param    &lt;key>    [&lt;optional_default_value>]


You may omit the optional default value if None is manageable in your test.
Otherwise, the second parameter will define a default value used if no parameter value is
defined for your key. This will happen if :

The test is executed outside of the runner
There is no parameter value for the key you have used
(for exemple, no value for this test case)


Known limitations¶
If you use python2, encoding will not be properly managed in parameter values.
This problem does not exist if you run the same tests using python3. As python2 will reached
its end of life on next january (official EOL date 2020/01/01), we recommend using python3.



Keywords¶










${value}=
Get Global Param
key
 

${value}=
Get Global Param
key
default_value



This keyword retrieves the value of a parameter defined for all tests executed in this run.
If there is a global value mapped on this key, the value argument is assigned with it,
otherwise it is assigned with the default value. If no default value was given, value
becomes None.
Ex :
${parameter}=    Get Global Param    target_base_url    http:localhost:8080/sut/











${value}=
Get Test Param
key
 

${value}=
Get Test Param
key
default_value



This keyword retrieves the value of a parameter defined for one execution of a test case
in this run.
This means that the run, as built through the json test suite file build by Squash TM,
may schedule several execution of the same test case with different value for the parameter.
If there is a value mapped on this key for the specific test case run,
the value argument is assigned with it,otherwise it is assigned with the default value.
If no default value was given, value becomes None.
Ex :
${login}=    Get Test Param    TC_login    test_user











${value}=
Get Param
key
 

${value}=
Get Param
key
default_value



This keyword combines both scopes. If a parameter value is mapped on this key
for this test case run, then the value argument is assigned with this value.
If not, global parameters are queried. If a global parameter value is mapped on the given key,
then the value argument is assigned with this value. Otherwise, value becomes None.
Ex :
${login}=    Get Param    login    test_user







Manually providing a .json file¶
If you want to manually provide a .json file, you need to add the following parameter on your runner command line
-Dtf.test.suite={file:path/to/json/FileName.json}.
“path/to/json/FileName.json” must be the relative path of your .json file from the root of your project.
If the .json file is located directly at the root of your project, just type -Dtf.test.suite={file:FileName.json}
For example :
mvn org.squashtest.ta.galaxia:squash-tf-robotframework-runner-maven-plugin:1.0.0-RELEASE:run -Dtf.test.suite={file:testSuite.json}


{
    &quot;test&quot;: [{
            &quot;id&quot;: &quot;39&quot;,
            &quot;script&quot;: &quot;My Test Suite/First Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;TC_UUID&quot;: &quot;3a7099ff-ab59-4e99-b21d-07e7d71d1ed5&quot;
            }
        }, {
            &quot;id&quot;: &quot;40&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Bertrand&quot;,
                &quot;DSNAME&quot;: &quot;dataset1&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;41&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-d836370d519b&quot;
            }
        }, {
            &quot;id&quot;: &quot;41&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Damien&quot;,
                &quot;DSNAME&quot;: &quot;dataset2&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;undefined&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-a036bf0d519b&quot;
            }
        }],
    &quot;param&quot;: {
        &quot;globalParamSection&quot;: &quot;This is global param section&quot;,
        &quot;user&quot;: &quot;foo&quot;,
        &quot;ow_ner.Na-me&quot;: &quot;bar&quot;,
    }
}



' or . = '
TF Param Service¶
This component provided by the squash-tf-services library allows you to retrieve the values
of the parameters transmitted to the runner through the json test suite file
(typically provided by Squash TM) in order to use them in your robotframework tests.
All keywords defined by the library allow the caller to define default values in the test,
so that it may be run outside of the runner.

Configuration¶

In order to use the TFParamService keywords in your test suite, you have to

install the python library. We assume here that the pip python package manager is installed.
python -m pip install squash-tf-services


Make sure that this command has been executed in all environement where the tests will be run,
in the relevant python environement. If you use a symlink and path setting in a linux environment
to use python3 as your ‘python’ binary, make sure this setting is in effect when you run the install
command.

Import the keyword library by using the following Library statement in your test :
Library squash_tf.TFParamService








Using parameter values in your tests¶
To retrieve parameter values in your tests, use one of the three defined keywords in the following way :
${value}= Get Param    &lt;key>    [&lt;optional_default_value>]


You may omit the optional default value if None is manageable in your test.
Otherwise, the second parameter will define a default value used if no parameter value is
defined for your key. This will happen if :

The test is executed outside of the runner
There is no parameter value for the key you have used
(for exemple, no value for this test case)


Known limitations¶
If you use python2, encoding will not be properly managed in parameter values.
This problem does not exist if you run the same tests using python3. As python2 will reached
its end of life on next january (official EOL date 2020/01/01), we recommend using python3.



Keywords¶










${value}=
Get Global Param
key
 

${value}=
Get Global Param
key
default_value



This keyword retrieves the value of a parameter defined for all tests executed in this run.
If there is a global value mapped on this key, the value argument is assigned with it,
otherwise it is assigned with the default value. If no default value was given, value
becomes None.
Ex :
${parameter}=    Get Global Param    target_base_url    http:localhost:8080/sut/











${value}=
Get Test Param
key
 

${value}=
Get Test Param
key
default_value



This keyword retrieves the value of a parameter defined for one execution of a test case
in this run.
This means that the run, as built through the json test suite file build by Squash TM,
may schedule several execution of the same test case with different value for the parameter.
If there is a value mapped on this key for the specific test case run,
the value argument is assigned with it,otherwise it is assigned with the default value.
If no default value was given, value becomes None.
Ex :
${login}=    Get Test Param    TC_login    test_user











${value}=
Get Param
key
 

${value}=
Get Param
key
default_value



This keyword combines both scopes. If a parameter value is mapped on this key
for this test case run, then the value argument is assigned with this value.
If not, global parameters are queried. If a global parameter value is mapped on the given key,
then the value argument is assigned with this value. Otherwise, value becomes None.
Ex :
${login}=    Get Param    login    test_user







Manually providing a .json file¶
If you want to manually provide a .json file, you need to add the following parameter on your runner command line
-Dtf.test.suite={file:path/to/json/FileName.json}.
“path/to/json/FileName.json” must be the relative path of your .json file from the root of your project.
If the .json file is located directly at the root of your project, just type -Dtf.test.suite={file:FileName.json}
For example :
mvn org.squashtest.ta.galaxia:squash-tf-robotframework-runner-maven-plugin:1.0.0-RELEASE:run -Dtf.test.suite={file:testSuite.json}


{
    &quot;test&quot;: [{
            &quot;id&quot;: &quot;39&quot;,
            &quot;script&quot;: &quot;My Test Suite/First Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;TC_UUID&quot;: &quot;3a7099ff-ab59-4e99-b21d-07e7d71d1ed5&quot;
            }
        }, {
            &quot;id&quot;: &quot;40&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Bertrand&quot;,
                &quot;DSNAME&quot;: &quot;dataset1&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;41&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-d836370d519b&quot;
            }
        }, {
            &quot;id&quot;: &quot;41&quot;,
            &quot;script&quot;: &quot;My Test Suite/Second Test Case&quot;,
            &quot;param&quot;: {
                &quot;TC_REFERENCE&quot;: &quot;&quot;,
                &quot;DS_name&quot;: &quot;Damien&quot;,
                &quot;DSNAME&quot;: &quot;dataset2&quot;,
                &quot;TC_CUF_CUF_CUSTOM&quot;: &quot;true&quot;,
                &quot;DS_age&quot;: &quot;undefined&quot;,
                &quot;TC_UUID&quot;: &quot;adec6164-5dec-4c8c-a0ae-a036bf0d519b&quot;
            }
        }],
    &quot;param&quot;: {
        &quot;globalParamSection&quot;: &quot;This is global param section&quot;,
        &quot;user&quot;: &quot;foo&quot;,
        &quot;ow_ner.Na-me&quot;: &quot;bar&quot;,
    }
}



')]</value>
   </webElementXpaths>
</WebElementEntity>
