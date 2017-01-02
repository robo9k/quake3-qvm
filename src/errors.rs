use nom;

error_chain!{
    errors {
        #[doc="A parsing error from `nom`"]
        Parser(e: nom::IError) {
            description("parsing error")
            display("parsing error: {:?}", e)
        }
    }
}
