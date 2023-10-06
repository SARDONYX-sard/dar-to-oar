"use client";

import { Box, Button, TextField } from "@mui/material";
import { useDynStyle, useStorageState } from "@/hooks";

export default function Settings() {
  const [style, setStyle] = useStorageState("customCSS");
  useDynStyle(style);

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
      <Button variant="outlined" onClick={() => setStyle(sample)}>
        use Sample
      </Button>
    </Box>
  );
}

const sample = `body {
    background-attachment: fixed;
    background-image: url("https://i.redd.it/red-forest-1920-1080-v0-s9u8ki2rr70a1.jpg?s=139edf608c428656505a143635a0687dec086229");
    background-repeat: no-repeat;
    background-size: cover;
}

main > form > div > div,
main > div {
    background-color: #2424248c;
}
`;
