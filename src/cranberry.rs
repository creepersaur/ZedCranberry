use zed_extension_api::{process::Command, *};

#[allow(unused)]
struct CranberryExtension;

impl Extension for CranberryExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed_extension_api::Command, String> {
        let lsp_binary = std::path::PathBuf::from("/Users/vivekmathur/cranberry-lsp/target/debug/cranberry-lsp");

        Ok(Command {
            command: lsp_binary.to_string_lossy().into(),
            args: vec![], // Common for LSP stdio transport
            env: vec![],                  // Optional: override env vars
        })
    }

    // Your existing method—now it will be called once completions flow from the LSP.
    // fn label_for_completion(
    //     &self,
    //     _language_server_id: &LanguageServerId,
    //     _completion: Completion,
    // ) -> Option<CodeLabel> {
    //     // This will customize *every* completion label for your LS.
    //     // In a real impl, match on _completion.label or .detail for selective styling.
    //     Some(CodeLabel {
    //         code: String::from("Hello"),
    //         spans: vec![CodeLabelSpan::Literal(CodeLabelSpanLiteral {
    //             text: "Hello".to_string(),
    //             highlight_name: Some("wow".to_string()), // Links to a theme highlight
    //         })],
    //         filter_range: Range { start: 0, end: 5 },
    //     })
    // }
}

register_extension!(CranberryExtension);
