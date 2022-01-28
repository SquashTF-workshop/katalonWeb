Tests sur interface Web avec Katalon.

Structure du projet:

Everything (TestSuite Collection)
|
|-NoParam (TestSuite)
  |
  |-JUnitKO (TestCase)
  |
  |-RobotOK (TestCase)
|  
|-FullParam (TestSuite)
  |
  |-SKFCommunity (TestCase)
  |
  |-SKFPremium (TestCase)
  |
  |-ExistingParameter (TestCase)
  
Liste des paramètres (community):

DSNAME : Logging

DS_list : List tests

TC_CUF_run : Run tests

DS_value : 15 (supérieur à 10)

Liste des paramètres (premium):

IT_CUF_db_title : Database Plugin

CPG_CUF_fc_title : Filechecker Plugin

TS_CUF_ftp_title : FTP Plugin

