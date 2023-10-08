"use client";

import { Box, TextField } from "@mui/material";
import { useDynStyle } from "@/hooks";
import { SelectStyleList } from "./../style_list";

export default function Settings() {
  const [style, setStyle] = useDynStyle();

  return (
    <Box
      component="main"
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        width: "100%",
      }}
    >
      <TextField
        sx={{
          marginTop: "20px",
          width: "80%",
          maxHeight: "30%",
        }}
        rows={10}
        label="Custom CSS"
        margin="dense"
        multiline
        onChange={(e) => {
          setStyle(e.target.value);
        }}
        placeholder="{ body: url('https://localhost' }"
        value={style}
      />

      <Box
        sx={{
          display: "flex",
          justifyContent: "space-around",
          width: "80%",
          marginTop: "20px",
          maxHeight: "30%",
        }}
      >
        <SelectStyleList setStyle={setStyle} />
      </Box>
    </Box>
  );
}
