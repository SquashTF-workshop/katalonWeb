<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Docs               Squash TF Runners   _0e9efa</name>
   <tag></tag>
   <elementGuidId>6cb33fb9-3a7b-40a3-9834-53f76821c2b1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Squash TF components'])[2]/following::div[1]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.wy-nav-content</value>
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
      <value>wy-nav-content</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
        
        
          

















  
    
      Docs »
        
      Squash TF Runners
    
    
      
        
            
            
               Edit on Bitbucket
            
          
        
      
    
  

  
  

          
           
            
  
Squash TF Runners¶


Java Junit Runner
Cucumber Java Runner
Robot Framework Runner



Environment configuration¶

Hint
Our runners are build to run in our Execution Server. So it’s highly recommended to configure your development
environment as describe in the section below.


First we advise to use the same tools versions as those we used in our execution server :

maven: 3.5.0
java: 1.8


Our maven library are hosted on our own repository. In consequence, you have to define our repository in your maven settings.
To do so, edit (or create) the maven settings.xml file in your .m2 directory (The .m2 directory is generally
located in your Home directory) and add a new profile:
&lt;settings>

...

  &lt;profiles>
    &lt;profile>
      &lt;id>tf-maven-repos&lt;/id>
      &lt;!-- Squash TF maven repository -->
      &lt;repositories>
        &lt;repository>
          &lt;id>org.squashtest.tf.release&lt;/id>
          &lt;name>squashtest test factory - releases&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
        &lt;/repository>
      &lt;/repositories>

      &lt;!-- Squash TF maven plugin repository -->
      &lt;pluginRepositories>
        &lt;pluginRepository>
          &lt;id>org.squashtest.plugins.release&lt;/id>
          &lt;name>squashtest.org&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
          &lt;snapshots>
            &lt;enabled>false&lt;/enabled>
          &lt;/snapshots>
          &lt;releases>
            &lt;enabled>true&lt;/enabled>
          &lt;/releases>
        &lt;/pluginRepository>
      &lt;/pluginRepositories>
    &lt;/profile>
  &lt;/profiles>

  &lt;activeProfiles>
    &lt;activeProfile>tf-maven-repos&lt;/activeProfile>
  &lt;/activeProfiles>

&lt;/settings>



We also advise to patch your maven by using the procedure below for a better logging with our runners :

Note
In all the procedure $MVN_HOME is your maven installation directory, and $MVN_VERSION your maven version.


Add in $MVN_HOME/lib/ext/ the jars:
log4j-slf4j-impl-2.5.jar
log4j-core2.5.jar
log4j-api-2.5.jar




Create a logging configuration file called log4j2.xml in $MVN_HOME/conf/logging/ and fill it with :

&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot; ?>
&lt;Configuration>
  &lt;Properties>
    &lt;Property name=&quot;maven.logging.root.level&quot;>INFO&lt;/Property>
  &lt;/Properties>
  &lt;Appenders>
    &lt;Console name=&quot;console&quot; target=&quot;SYSTEM_OUT&quot;>
      &lt;PatternLayout pattern=&quot;[%p] %msg%n%throwable&quot; />
    &lt;/Console>
  &lt;/Appenders>
  &lt;Loggers>
    &lt;Root level=&quot;${sys:maven.logging.root.level}&quot;>
      &lt;Appender-ref ref=&quot;console&quot;/>
    &lt;/Root>
&lt;!-- &lt;logger name=&quot;[USER_MESSAGE]&quot; level=&quot;DEBUG&quot;/> -->
  &lt;/Loggers>
&lt;/Configuration>



Remove if exists :
In the directory $MVN_HOME/lib the file maven-sl4j-provider-$MVN_VERSION.jar
In the directory $MVN_HOME/conf/logging/ the file deletesimpleLogger.properties







           
           
          
          
  
    
      
        Next 
      
      
         Previous
      
    
  

  

  
    
        © Copyright 2018 - 2020, Henix
      
        
          Revision cc7f5832.
        
      

    
  
  Built with Sphinx using a theme provided by Read the Docs. 



        
      </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[@class=&quot;wy-body-for-nav&quot;]/div[@class=&quot;wy-grid-for-nav&quot;]/section[@class=&quot;wy-nav-content-wrap&quot;]/div[@class=&quot;wy-nav-content&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Squash TF components'])[2]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Monetize your site'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = '
        
        
        
          

















  
    
      Docs »
        
      Squash TF Runners
    
    
      
        
            
            
               Edit on Bitbucket
            
          
        
      
    
  

  
  

          
           
            
  
Squash TF Runners¶


Java Junit Runner
Cucumber Java Runner
Robot Framework Runner



Environment configuration¶

Hint
Our runners are build to run in our Execution Server. So it’s highly recommended to configure your development
environment as describe in the section below.


First we advise to use the same tools versions as those we used in our execution server :

maven: 3.5.0
java: 1.8


Our maven library are hosted on our own repository. In consequence, you have to define our repository in your maven settings.
To do so, edit (or create) the maven settings.xml file in your .m2 directory (The .m2 directory is generally
located in your Home directory) and add a new profile:
&lt;settings>

...

  &lt;profiles>
    &lt;profile>
      &lt;id>tf-maven-repos&lt;/id>
      &lt;!-- Squash TF maven repository -->
      &lt;repositories>
        &lt;repository>
          &lt;id>org.squashtest.tf.release&lt;/id>
          &lt;name>squashtest test factory - releases&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
        &lt;/repository>
      &lt;/repositories>

      &lt;!-- Squash TF maven plugin repository -->
      &lt;pluginRepositories>
        &lt;pluginRepository>
          &lt;id>org.squashtest.plugins.release&lt;/id>
          &lt;name>squashtest.org&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
          &lt;snapshots>
            &lt;enabled>false&lt;/enabled>
          &lt;/snapshots>
          &lt;releases>
            &lt;enabled>true&lt;/enabled>
          &lt;/releases>
        &lt;/pluginRepository>
      &lt;/pluginRepositories>
    &lt;/profile>
  &lt;/profiles>

  &lt;activeProfiles>
    &lt;activeProfile>tf-maven-repos&lt;/activeProfile>
  &lt;/activeProfiles>

&lt;/settings>



We also advise to patch your maven by using the procedure below for a better logging with our runners :

Note
In all the procedure $MVN_HOME is your maven installation directory, and $MVN_VERSION your maven version.


Add in $MVN_HOME/lib/ext/ the jars:
log4j-slf4j-impl-2.5.jar
log4j-core2.5.jar
log4j-api-2.5.jar




Create a logging configuration file called log4j2.xml in $MVN_HOME/conf/logging/ and fill it with :

&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot; ?>
&lt;Configuration>
  &lt;Properties>
    &lt;Property name=&quot;maven.logging.root.level&quot;>INFO&lt;/Property>
  &lt;/Properties>
  &lt;Appenders>
    &lt;Console name=&quot;console&quot; target=&quot;SYSTEM_OUT&quot;>
      &lt;PatternLayout pattern=&quot;[%p] %msg%n%throwable&quot; />
    &lt;/Console>
  &lt;/Appenders>
  &lt;Loggers>
    &lt;Root level=&quot;${sys:maven.logging.root.level}&quot;>
      &lt;Appender-ref ref=&quot;console&quot;/>
    &lt;/Root>
&lt;!-- &lt;logger name=&quot;[USER_MESSAGE]&quot; level=&quot;DEBUG&quot;/> -->
  &lt;/Loggers>
&lt;/Configuration>



Remove if exists :
In the directory $MVN_HOME/lib the file maven-sl4j-provider-$MVN_VERSION.jar
In the directory $MVN_HOME/conf/logging/ the file deletesimpleLogger.properties







           
           
          
          
  
    
      
        Next 
      
      
         Previous
      
    
  

  

  
    
        © Copyright 2018 - 2020, Henix
      
        
          Revision cc7f5832.
        
      

    
  
  Built with Sphinx using a theme provided by Read the Docs. 



        
      ' or . = '
        
        
        
          

















  
    
      Docs »
        
      Squash TF Runners
    
    
      
        
            
            
               Edit on Bitbucket
            
          
        
      
    
  

  
  

          
           
            
  
Squash TF Runners¶


Java Junit Runner
Cucumber Java Runner
Robot Framework Runner



Environment configuration¶

Hint
Our runners are build to run in our Execution Server. So it’s highly recommended to configure your development
environment as describe in the section below.


First we advise to use the same tools versions as those we used in our execution server :

maven: 3.5.0
java: 1.8


Our maven library are hosted on our own repository. In consequence, you have to define our repository in your maven settings.
To do so, edit (or create) the maven settings.xml file in your .m2 directory (The .m2 directory is generally
located in your Home directory) and add a new profile:
&lt;settings>

...

  &lt;profiles>
    &lt;profile>
      &lt;id>tf-maven-repos&lt;/id>
      &lt;!-- Squash TF maven repository -->
      &lt;repositories>
        &lt;repository>
          &lt;id>org.squashtest.tf.release&lt;/id>
          &lt;name>squashtest test factory - releases&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
        &lt;/repository>
      &lt;/repositories>

      &lt;!-- Squash TF maven plugin repository -->
      &lt;pluginRepositories>
        &lt;pluginRepository>
          &lt;id>org.squashtest.plugins.release&lt;/id>
          &lt;name>squashtest.org&lt;/name>
          &lt;url>http://repo.squashtest.org/maven2/releases&lt;/url>
          &lt;snapshots>
            &lt;enabled>false&lt;/enabled>
          &lt;/snapshots>
          &lt;releases>
            &lt;enabled>true&lt;/enabled>
          &lt;/releases>
        &lt;/pluginRepository>
      &lt;/pluginRepositories>
    &lt;/profile>
  &lt;/profiles>

  &lt;activeProfiles>
    &lt;activeProfile>tf-maven-repos&lt;/activeProfile>
  &lt;/activeProfiles>

&lt;/settings>



We also advise to patch your maven by using the procedure below for a better logging with our runners :

Note
In all the procedure $MVN_HOME is your maven installation directory, and $MVN_VERSION your maven version.


Add in $MVN_HOME/lib/ext/ the jars:
log4j-slf4j-impl-2.5.jar
log4j-core2.5.jar
log4j-api-2.5.jar




Create a logging configuration file called log4j2.xml in $MVN_HOME/conf/logging/ and fill it with :

&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot; ?>
&lt;Configuration>
  &lt;Properties>
    &lt;Property name=&quot;maven.logging.root.level&quot;>INFO&lt;/Property>
  &lt;/Properties>
  &lt;Appenders>
    &lt;Console name=&quot;console&quot; target=&quot;SYSTEM_OUT&quot;>
      &lt;PatternLayout pattern=&quot;[%p] %msg%n%throwable&quot; />
    &lt;/Console>
  &lt;/Appenders>
  &lt;Loggers>
    &lt;Root level=&quot;${sys:maven.logging.root.level}&quot;>
      &lt;Appender-ref ref=&quot;console&quot;/>
    &lt;/Root>
&lt;!-- &lt;logger name=&quot;[USER_MESSAGE]&quot; level=&quot;DEBUG&quot;/> -->
  &lt;/Loggers>
&lt;/Configuration>



Remove if exists :
In the directory $MVN_HOME/lib the file maven-sl4j-provider-$MVN_VERSION.jar
In the directory $MVN_HOME/conf/logging/ the file deletesimpleLogger.properties







           
           
          
          
  
    
      
        Next 
      
      
         Previous
      
    
  

  

  
    
        © Copyright 2018 - 2020, Henix
      
        
          Revision cc7f5832.
        
      

    
  
  Built with Sphinx using a theme provided by Read the Docs. 



        
      ')]</value>
   </webElementXpaths>
</WebElementEntity>
