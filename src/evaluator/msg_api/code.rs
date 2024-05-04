/// Message passing API codes for communication between server and client pkl
/// see https://pkl-lang.org/main/current/bindings-specification/index.html for
/// more details

//TODO this should be an enum
pub const CODE_NEW_EVALUATOR                 :u8 = 0x20;
pub const CODE_NEW_EVALUATOR_RESPONSE        :u8 = 0x21;
pub const CODE_CLOSE_EVALUATOR               :u8 = 0x22;
pub const CODE_EVALUATE                      :u8 = 0x23;
pub const CODE_EVALUATE_RESPONSE             :u8 = 0x24;
pub const CODE_EVALUATE_LOG                  :u8 = 0x25;
pub const CODE_EVALUATE_READ                 :u8 = 0x26;
pub const CODE_EVALUATE_READ_RESPONSE        :u8 = 0x27;
pub const CODE_EVALUATE_READ_MODULE          :u8 = 0x28;
pub const CODE_EVALUATE_READ_MODULE_RESPONSE :u8 = 0x29;
pub const CODE_LIST_RESOURCES_REQUEST        :u8 = 0x2a;
pub const CODE_LIST_RESOURCES_RESPONSE       :u8 = 0x2b;
pub const CODE_LIST_MODULES_REQUEST          :u8 = 0x2c;
pub const CODE_LIST_MODULES_RESPONSE         :u8 = 0x2d;

#[derive(Debug)]
pub enum MessageCode {
    NewEvaluator                = 0x20,
    NewEvaluatorResponse        = 0x21,
    CloseEvaluator              = 0x22,
    Evaluate                    = 0x23,
    EvaluateResponse            = 0x24,
    EvaluateLog                 = 0x25,
    EvaluateRead                = 0x26,
    EvaluateReadResponse        = 0x27,
    EvaluateReadModule          = 0x28,
    EvaluateReadModuleResponse  = 0x29,
    ListResourcesRequest        = 0x2a,
    ListResourcesResponse       = 0x2b,
    ListModulesRequest          = 0x2c,
    ListModulesResponse         = 0x2d,
}

impl TryFrom<u8> for MessageCode {
    type Error = &'static str;

    fn try_from(source: u8) -> Result<Self, Self::Error> {
        match source {
            0x20 => Ok(Self::NewEvaluator),
            0x21 => Ok(Self::NewEvaluatorResponse),
            0x22 => Ok(Self::CloseEvaluator),
            0x23 => Ok(Self::Evaluate),
            0x24 => Ok(Self::EvaluateResponse),
            0x25 => Ok(Self::EvaluateLog),
            0x26 => Ok(Self::EvaluateRead),
            0x27 => Ok(Self::EvaluateReadResponse),
            0x28 => Ok(Self::EvaluateReadModule),
            0x29 => Ok(Self::EvaluateReadModuleResponse),
            0x2a => Ok(Self::ListResourcesRequest),
            0x2b => Ok(Self::ListResourcesResponse),
            0x2c => Ok(Self::ListModulesRequest),
            0x2d => Ok(Self::ListModulesResponse),
            _    => Err("Value out of range"),
        }
    }
}
