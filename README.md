Tests sur interface Web avec Katalon.

Structure du projet:

```
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
```

Liste des paramètres (community):

DSNAME : Logging

DS_list : List tests

TC_CUF_run : Run tests

DS_value : 15 (supérieur à 10)

Liste des paramètres (premium):

IT_CUF_db_title : Database Plugin

CPG_CUF_fc_title : Filechecker Plugin

TS_CUF_ftp_title : FTP Plugin

Des tests supplémentaires ont été ajoutés (notamment les dossiers DataFromParam et DataSuite) :

La partie DataFromParam récupère la chaîne de caractère à comparer du paramètre DSNAME.

La partie DataSuite récupère la chaîne de caractère du fichier de données. Le cas de test est répété autant de fois qu'il y a de lignes dans le .csv (donc 4 fois).

Chaque partie dispose d'une collection reliée à un navigateur (un pour Firefox, un pour Chrome et un autre avec la suite exécutée une première sur l'un puis sur l'autre

Les valeurs acceptées sont celles de la liste principale à l'adresse suivante : https://skf.readthedocs.io/projects/skf/en/doc-stable/libraries/plugins.html
