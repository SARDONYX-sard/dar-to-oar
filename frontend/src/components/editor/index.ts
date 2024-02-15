// @index('./*', f => `export * from '${f.path}'`)
export * from './css_editor';
export * from './js_editor';

// NOTE: These extensions must be loaded after importing AceEditor or they will error
import 'ace-builds/src-min-noconflict/ext-language_tools'; // For completion
import 'ace-builds/src-min-noconflict/keybinding-vim';
import 'ace-builds/src-min-noconflict/mode-css';
import 'ace-builds/src-min-noconflict/mode-javascript';
import 'ace-builds/src-min-noconflict/snippets/css';
import 'ace-builds/src-min-noconflict/snippets/javascript';
import 'ace-builds/src-min-noconflict/theme-one_dark';
