/// Message passing API codes for communication between server and client pkl
/// see https://pkl-lang.org/main/current/bindings-specification/index.html for
/// more details


const CODE_NEW_EVALUATOR                 :u8 = 0x20;
const CODE_NEW_EVALUATOR_RESPONSE        :u8 = 0x21;
const CODE_CLOSE_EVALUATOR               :u8 = 0x22;
const CODE_EVALUATE                      :u8 = 0x23;
const CODE_EVALUATE_RESPONSE             :u8 = 0x24;
const CODE_EVALUATE_LOG                  :u8 = 0x25;
const CODE_EVALUATE_READ                 :u8 = 0x26;
const CODE_EVALUATE_READ_RESPONSE        :u8 = 0x27;
const CODE_EVALUATE_READ_MODULE          :u8 = 0x28;
const CODE_EVALUATE_READ_MODULE_RESPONSE :u8 = 0x29;
const CODE_LIST_RESOURCES_REQUEST        :u8 = 0x2a;
const CODE_LIST_RESOURCES_RESPONSE       :u8 = 0x2b;
const CODE_LIST_MODULES_REQUEST          :u8 = 0x2c;
const CODE_LIST_MODULES_RESPONSE         :u8 = 0x2d;
