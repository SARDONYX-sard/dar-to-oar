"use client";

import "ace-builds/src-noconflict/ext-code_lens";
import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/keybinding-vim";
import "ace-builds/src-noconflict/mode-css";
import "ace-builds/src-noconflict/mode-javascript";
import "ace-builds/src-noconflict/snippets/css";
import "ace-builds/src-noconflict/snippets/javascript";
import "ace-builds/src-noconflict/theme-one_dark";
import AceEditor from "react-ace";
import InputLabel from "@mui/material/InputLabel";
import packageJson from "@/../../package.json";
import { Box } from "@mui/material";
import { SelectEditorMode } from "@/components/lists/editor_list";
import { StyleList } from "@/components/lists/style_list";
import { Toaster } from "react-hot-toast";
import { selectEditorMode, type EditorMode } from "@/utils/editor";
import { useDynStyle, useInjectScript, useStorageState } from "@/hooks";

export default function Settings() {
  const [editorMode, setEditorMode] = useStorageState("editorMode", "default");
  const [preset, setPreset] = useStorageState("presetNumber", "0");
  const [style, setStyle] = useDynStyle();

  const setEditorKeyMode = (editorMode: EditorMode) => {
    setEditorMode(editorMode ?? "");
  };

  return (
    <Box
      component="main"
      sx={{
        display: "flex",
        marginBottom: "1rem",
        flexDirection: "column",
        alignItems: "center",
        width: "100%",
      }}
    >
      <Toaster position="bottom-right" reverseOrder={false} />
      <CSSEditor
        editorMode={editorMode}
        setPreset={setPreset}
        setStyle={setStyle}
        style={style}
      />

      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          width: "40%",
          marginTop: "30px",
        }}
      >
        <SelectEditorMode
          editorMode={selectEditorMode(editorMode)}
          setEditorMode={setEditorKeyMode}
        />

        <StyleList preset={preset} setPreset={setPreset} setStyle={setStyle} />
      </div>

      <JSEditor editorMode={editorMode} />

      <Help />
    </Box>
  );
}

type CSSEditorProps = {
  editorMode: string;
  setPreset: (script: string) => void;
  setStyle: (style: string) => void;
  style: string;
};
const CSSEditor = ({
  editorMode,
  setPreset,
  setStyle,
  style,
}: CSSEditorProps) => {
  return (
    <>
      <InputLabel sx={{ marginTop: "20px" }}>Custom CSS</InputLabel>
      <AceEditor
        style={{
          width: "95%",
          backgroundColor: "#2424248c",
        }}
        onChange={(value) => {
          setStyle(value);
          localStorage.setItem("customCSS", value);
          setPreset("0");
        }}
        fontSize={"1rem"}
        height="300px"
        mode="css"
        theme="one_dark"
        value={style}
        setOptions={{ useWorker: false }}
        placeholder="{ body: url('https://localhost' }"
        name="Custom CSS"
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        keyboardHandler={selectEditorMode(editorMode)}
        highlightActiveLine
        tabSize={2}
        editorProps={{ $blockScrolling: true }}
      />
    </>
  );
};

type JSEditorProps = {
  editorMode: string;
};
const JSEditor = ({ editorMode }: JSEditorProps) => {
  const [script, setScript] = useInjectScript();

  return (
    <>
      <InputLabel error sx={{ marginTop: "20px" }}>
        Custom JavaScript(Please do not execute untrusted scripts)
      </InputLabel>
      <AceEditor
        style={{
          width: "95%",
          backgroundColor: "#2424248c",
        }}
        onChange={(value) => {
          localStorage.setItem("customJS", value);
          setScript(value);
        }}
        placeholder={`(()=> {
    const p = document.createElement('p');
    p.innerText = 'Hello';
    document.body.appendChild(p);
)()`}
        editorProps={{ $blockScrolling: true }}
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        fontSize={"1rem"}
        height="250px"
        highlightActiveLine
        keyboardHandler={selectEditorMode(editorMode)}
        mode="javascript"
        name="Custom JavaScript"
        setOptions={{ useWorker: false }}
        tabSize={2}
        theme="one_dark"
        value={script}
      />
    </>
  );
};

const Help = () => {
  const handleClick = () => open(packageJson.homepage);

  return (
    <div>
      <p>Version: {packageJson.version}</p>
      <p>
        Source Code:{" "}
        <a
          style={{ cursor: "pointer", color: "#00c2ff" }}
          onClick={handleClick}
          onKeyDown={handleClick}
        >
          GitHub
        </a>
      </p>
    </div>
  );
};
