"use client";

import AceEditor from "react-ace";
import type { EditorMode } from "@/utils/editor";
import { Box } from "@mui/material";
import { SelectEditorMode } from "@/components/editor_list";
import { SelectStyleList } from "@/components/style_list";
import { Toaster } from "react-hot-toast";
import { selectEditorMode } from "@/utils/editor";
import { useDynStyle, useInjectScript, useStorageState } from "@/hooks";

import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/keybinding-vim";
import "ace-builds/src-noconflict/mode-css";
import "ace-builds/src-noconflict/mode-javascript";
import "ace-builds/src-noconflict/theme-one_dark";
import InputLabel from "@mui/material/InputLabel";

export default function Settings() {
  const [style, setStyle] = useDynStyle();
  const [script, setScript] = useInjectScript();
  const [preset, setPreset] = useStorageState("presetNumber", "0");
  const [editorMode, setEditorMode] = useStorageState("editorMode", "default");

  const setEditorKeyMode = (editorMode: EditorMode) => {
    setEditorMode(editorMode ?? "");
  };

  return (
    <Box
      component="main"
      sx={{
        display: "flex",
        marginBottom: "20px",
        flexDirection: "column",
        alignItems: "center",
        width: "100%",
      }}
    >
      <Toaster position="bottom-right" reverseOrder={false} />
      <InputLabel sx={{ marginTop: "20px" }}>Custom CSS</InputLabel>
      <AceEditor
        style={{
          width: "95%",
          backgroundColor: "#2424248c",
        }}
        height="300px"
        mode="css"
        theme="one_dark"
        value={style}
        debounceChangePeriod={500}
        onChange={(value) => {
          setStyle(value);
          localStorage.setItem("customCSS", value);
          setPreset("0");
        }}
        placeholder="{ body: url('https://localhost' }"
        name="Custom CSS"
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        keyboardHandler={selectEditorMode(editorMode)}
        highlightActiveLine
        editorProps={{ $blockScrolling: true }}
      />

      <Box
        sx={{
          display: "flex",
          justifyContent: "space-around",
          width: "80%",
          marginTop: "20px",
          maxHeight: "20%",
        }}
      >
        <SelectStyleList
          preset={preset}
          setPreset={setPreset}
          setStyle={setStyle}
        />
        <SelectEditorMode
          editorMode={selectEditorMode(editorMode)}
          setEditorMode={setEditorKeyMode}
        />
      </Box>

      <InputLabel error sx={{ marginTop: "20px" }}>
        Custom JavaScript(Please do not execute untrusted scripts)
      </InputLabel>
      <AceEditor
        style={{
          width: "95%",
          backgroundColor: "#2424248c",
        }}
        height="200px"
        mode="javascript"
        theme="one_dark"
        value={script}
        onChange={(value) => {
          localStorage.setItem("customJS", value);
          setScript(value);
        }}
        placeholder="(()=> {
    const p = document.createElement('p');
    p.innerText = 'Hello';
    document.body.appendChild(p);
)()"
        name="Custom JavaScript"
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        keyboardHandler={selectEditorMode(editorMode)}
        highlightActiveLine
        editorProps={{ $blockScrolling: true }}
      />
    </Box>
  );
}
