// Image Modes
pub const IM_BILEVEL: u16 = 1u16;
pub const IM_PALETTED: u16 = 2u16;
pub const IM_GRAY: u16 = 3u16;
pub const IM_GRAYINVERT: u16 = 4u16;
pub const IM_RGB: u16 = 5u16;
pub const IM_RGBA: u16 = 6u16;
pub const IM_NRGBA: u16 = 7u16;

pub const COMPRESS_NONE: u16 = 1;
pub const COMPRESS_CCITT: u16 = 2;
pub const COMPRESS_G3: u16 = 3; // Group 3 Fax.
pub const COMPRESS_G4: u16 = 4; // Group 4 Fax.
pub const COMPRESS_LZW: u16 = 5;
pub const COMPRESS_JPEGOLD: u16 = 6; // Superseded by cJPEG.
pub const COMPRESS_JPEG: u16 = 7;
pub const COMPRESS_DEFLATE: u16 = 8; // zlib compression.
pub const COMPRESS_PACKBITS: u16 = 32773;
pub const COMPRESS_DEFLATEOLD: u16 = 32946; // Superseded by cDeflate.

pub const DT_BYTE: u16 = 1;
pub const DT_ASCII: u16 = 2;
pub const DT_SHORT: u16 = 3;
pub const DT_LONG: u16 = 4;
pub const DT_RATIONAL: u16 = 5;
pub const DT_SBYTE: u16 = 6;
pub const DT_UNDEFINED: u16 = 7;
pub const DT_SSHORT: u16 = 8;
pub const DT_SLONG: u16 = 9;
pub const DT_SRATIONAL: u16 = 10;
pub const DT_FLOAT: u16 = 11;
pub const DT_DOUBLE: u16 = 12;
pub const DT_TIFF_LONG8: u16 = 16; // bigTiff
pub const DT_TIFF_SLONG8: u16 = 17; // bigTiff
pub const DT_TIFF_IFD8: u16 = 18; // bigTiff

pub const PI_WHITEISZERO: u16 = 0;
pub const PI_BLACKISZERO: u16 = 1;
pub const PI_RGB: u16 = 2;
pub const PI_PALETTED: u16 = 3;
// pub const PI_TRANSMASK: u16   = 4; // transparency mask
// const PI_CMYK: u16        = 5;
// const PI_YCBCR: u16       = 6;
// const PI_CIELAB: u16      = 8;

// Tags (see p. 28-41 of the spec).
pub const TAG_NEWSUBFILETYPE: u16 = 254u16;
pub const TAG_IMAGEWIDTH: u16 = 256u16;
pub const TAG_IMAGELENGTH: u16 = 257u16;
pub const TAG_BITSPERSAMPLE: u16 = 258u16;
pub const TAG_COMPRESSION: u16 = 259u16;
pub const TAG_PHOTOMETRICINTERPRETATION: u16 = 262u16;
pub const TAG_FILLORDER: u16 = 266u16;
pub const TAG_DOCUMENTNAME: u16 = 269u16;

pub const TAG_STRIPOFFSETS: u16 = 273u16;
pub const TAG_ORIENTATION: u16 = 274u16;
pub const TAG_SAMPLESPERPIXEL: u16 = 277u16;
pub const TAG_ROWSPERSTRIP: u16 = 278u16;
pub const TAG_STRIPBYTECOUNTS: u16 = 279u16;
pub const TAG_MINSAMPLEVALUE: u16 = 280u16;
pub const TAG_MAXSAMPLEVALUE: u16 = 281u16;
pub const TAG_PLANARCONFIGURATION: u16 = 284u16;

pub const TAG_TILEWIDTH: u16 = 322u16;
pub const TAG_TILELENGTH: u16 = 323u16;
pub const TAG_TILEOFFSETS: u16 = 324u16;
pub const TAG_TILEBYTECOUNTS: u16 = 325u16;

pub const TAG_XRESOLUTION: u16 = 282u16;
pub const TAG_YRESOLUTION: u16 = 283u16;
pub const TAG_RESOLUTIONUNIT: u16 = 296u16;

pub const TAG_SOFTWARE: u16 = 305u16;
pub const TAG_PREDICTOR: u16 = 317u16;
pub const TAG_COLORMAP: u16 = 320u16;
pub const TAG_EXTRASAMPLES: u16 = 338u16;
pub const TAG_SAMPLEFORMAT: u16 = 339u16;
pub const TAG_JPEGTABLES: u16 = 347u16;

pub const TIFFTAG_REFERENCEBLACKWHITE: u16 = 532u16;

pub const TAG_GDAL_METADATA: u16 = 42112u16;
pub const TAG_GDAL_NODATA: u16 = 42113u16;

pub const TAG_MODELPIXELSCALETAG: u16 = 33550u16;
pub const TAG_MODELTRANSFORMATIONTAG: u16 = 34264u16;
pub const TAG_MODELTIEPOINTTAG: u16 = 33922u16;
pub const TAG_GEOKEYDIRECTORYTAG: u16 = 34735u16;
pub const TAG_GEODOUBLEPARAMSTAG: u16 = 34736u16;
pub const TAG_GEOASCIIPARAMSTAG: u16 = 34737u16;
pub const TAG_INTERGRRAPH_PACKET_DATA: u16 = 33918u16;
pub const TAG_INTERGRAPHMATRIXTAG: u16 = 33920u16;

pub const TAG_GTMODELTYPEGEOKEY: u16 = 1024u16;
pub const TAG_GTRASTERTYPEGEOKEY: u16 = 1025u16;
pub const TAG_GTCITATIONGEOKEY: u16 = 1026u16;
pub const TAG_GEOGRAPHICTYPEGEOKEY: u16 = 2048u16;
pub const TAG_GEOGCITATIONGEOKEY: u16 = 2049u16;
pub const TAG_GEOGGEODETICDATUMGEOKEY: u16 = 2050u16;
pub const TAG_GEOGPRIMEMERIDIANGEOKEY: u16 = 2051u16;
pub const TAG_GEOGLINEARUNITSGEOKEY: u16 = 2052u16;
pub const TAG_GEOGLINEARUNITSIZEGEOKEY: u16 = 2053u16;
pub const TAG_GEOGANGULARUNITSGEOKEY: u16 = 2054u16;
pub const TAG_GEOGANGULARUNITSIZEGEOKEY: u16 = 2055u16;
pub const TAG_GEOGELLIPSOIDGEOKEY: u16 = 2056u16;
pub const TAG_GEOGSEMIMAJORAXISGEOKEY: u16 = 2057u16;
pub const TAG_GEOGSEMIMINORAXISGEOKEY: u16 = 2058u16;
pub const TAG_GEOGINVFLATTENINGGEOKEY: u16 = 2059u16;
pub const TAG_GEOGAZIMUTHUNITSGEOKEY: u16 = 2060u16;
pub const TAG_GEOGPRIMEMERIDIANLONGGEOKEY: u16 = 2061u16;
pub const TAG_PROJECTEDCSTYPEGEOKEY: u16 = 3072u16;
pub const TAG_PCSCITATIONGEOKEY: u16 = 3073u16;
pub const TAG_PROJECTIONGEOKEY: u16 = 3074u16;
pub const TAG_PROJCOORDTRANSGEOKEY: u16 = 3075u16;
pub const TAG_PROJLINEARUNITSGEOKEY: u16 = 3076u16;
pub const TAG_PROJLINEARUNITSIZEGEOKEY: u16 = 3077u16;
pub const TAG_PROJSTDPARALLEL1GEOKEY: u16 = 3078u16;
pub const TAG_PROJSTDPARALLEL2GEOKEY: u16 = 3079u16;
pub const TAG_PROJNATORIGINLONGGEOKEY: u16 = 3080u16;
pub const TAG_PROJNATORIGINLATGEOKEY: u16 = 3081u16;
pub const TAG_PROJFALSEEASTINGGEOKEY: u16 = 3082u16;
pub const TAG_PROJFALSENORTHINGGEOKEY: u16 = 3083u16;
pub const TAG_PROJFALSEORIGINLONGGEOKEY: u16 = 3084u16;
pub const TAG_PROJFALSEORIGINLATGEOKEY: u16 = 3085u16;
pub const TAG_PROJFALSEORIGINEASTINGGEOKEY: u16 = 3086u16;
pub const TAG_PROJFALSEORIGINNORTHINGGEOKEY: u16 = 3087u16;
pub const TAG_PROJCENTERLONGGEOKEY: u16 = 3088u16;
pub const TAG_PROJCENTERLATGEOKEY: u16 = 3089u16;
pub const TAG_PROJCENTEREASTINGGEOKEY: u16 = 3090u16;
pub const TAG_PROJCENTERNORTHINGGEOKEY: u16 = 3091u16;
pub const TAG_PROJSCALEATNATORIGINGEOKEY: u16 = 3092u16;
pub const TAG_PROJSCALEATCENTERGEOKEY: u16 = 3093u16;
pub const TAG_PROJAZIMUTHANGLEGEOKEY: u16 = 3094u16;
pub const TAG_PROJSTRAIGHTVERTPOLELONGGEOKEY: u16 = 3095u16;
pub const TAG_VERTICALCSTYPEGEOKEY: u16 = 4096u16;
pub const TAG_VERTICALCITATIONGEOKEY: u16 = 4097u16;
pub const TAG_VERTICALDATUMGEOKEY: u16 = 4098u16;
pub const TAG_VERTICALUNITSGEOKEY: u16 = 4099u16;

pub const TAG_PHOTOSHOP: u16 = 34377u16;
