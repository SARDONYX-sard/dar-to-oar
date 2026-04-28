import monaco from 'monaco-editor';
import { registerCompletionProvider } from './providers/completion';
import { clearDiagnostics, registerCodeLen } from './providers/diagnostic';
import { registerDocumentFormattingEditProvider } from './providers/formatter';
import { registerHoverProvider } from './providers/hover';
import { registerInlayHintsProvider } from './providers/inlay_hint';
import { registerMonarchTokensProvider } from './providers/monarch_token';
import { registerDocumentSemanticTokensProvider } from './providers/semantic_token';
import { registerSignatureHelpProvider } from './providers/signature';

export const MAPPING_TABLE_LANGUAGE_ID = 'mapping_table';
export type MappingTableLspOptions = {
  completion?: boolean;
  diagnostics?: boolean;
  formatter?: boolean;
  semanticTokens?: boolean;
  hover?: boolean;
  inlayHints?: boolean;
  signatureHelp?: boolean;
};
export const supportMappingTable = (options: MappingTableLspOptions = {}) => {
  return (editor: monaco.editor.IStandaloneCodeEditor, monacoEnv: typeof monaco): monaco.IDisposable[] => {
    monacoEnv.languages.register({ id: MAPPING_TABLE_LANGUAGE_ID });

    const disposables: monaco.IDisposable[] = [];

    if (options.diagnostics !== false) {
      disposables.push(...registerCodeLen(editor, monacoEnv));
    } else {
      clearDiagnostics(monacoEnv);
    }

    if (options.completion !== false) {
      disposables.push(registerCompletionProvider(monacoEnv));
    }

    if (options.formatter !== false) {
      disposables.push(registerDocumentFormattingEditProvider(monacoEnv));
    }

    if (options.semanticTokens !== false) {
      disposables.push(registerDocumentSemanticTokensProvider(monacoEnv));
    }

    if (options.hover !== false) {
      disposables.push(registerHoverProvider(monacoEnv));
    }

    if (options.inlayHints !== false) {
      disposables.push(registerInlayHintsProvider(monacoEnv));
    }

    registerMonarchTokensProvider(monacoEnv); // no dispose

    if (options.signatureHelp !== false) {
      disposables.push(registerSignatureHelpProvider(editor, monacoEnv));
    }

    return disposables;
  };
};
