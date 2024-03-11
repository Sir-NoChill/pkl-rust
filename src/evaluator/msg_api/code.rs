/// Message passing API codes for communication between server and client pkl
/// see https://pkl-lang.org/main/current/bindings-specification/index.html for
/// more details


static const CODE_NEW_EVALUATOR                 u8 = 0x20;
static const CODE_NEW_EVALUATOR_RESPONSE        u8 = 0x21;
static const CODE_CLOSE_EVALUATOR               u8 = 0x22;
static const CODE_EVALUATE                      u8 = 0x23;
static const CODE_EVALUATE_RESPONSE             u8 = 0x24;
static const CODE_EVALUATE_LOG                  u8 = 0x25;
static const CODE_EVALUATE_READ                 u8 = 0x26;
static const CODE_EVALUATE_READ_RESPONSE        u8 = 0x27;
static const CODE_EVALUATE_READ_MODULE          u8 = 0x28;
static const CODE_EVALUATE_READ_MODULE_RESPONSE u8 = 0x29;
static const CODE_LIST_RESOURCES_REQUEST        u8 = 0x2a;
static const CODE_LIST_RESOURCES_RESPONSE       u8 = 0x2b;
static const CODE_LIST_MODULES_REQUEST          u8 = 0x2c;
static const CODE_LIST_MODULES_RESPONSE         u8 = 0x2d;
