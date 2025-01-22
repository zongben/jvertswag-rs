# jvertswag-rs
cli tool for converting json to openapi jsdoc

## Usage
|args|short|optional|multi|description|default value|
|-|-|-|-|-|-|
|root|-R|false|false|url prefix e.g `http://localhost:80`|none|
|path|-P|false|false|url path e.g `/api/student/{studentId}`|none|
|method|-X|false|false|method e.g `post`|none|
|header|-H|false|true|url headers e.g `content-type: application/json`|none|
|query|-q|true|true|url query e.g `date=2025-01-01`|none|
|param|-p|true|true|url path params e.g `studentId=1`|none|
|body|-d|true|false|request body with json format|none|
|res|-r|true|false|response body with json format.|none|
|gap|-g|true|false|openapi structure gap, value is between 1 to 10|2|
|comment|-c|true|false|prefix comment|`""`|
|offset|-o|true|false|prefix offset gap|0|
