"use client";

import { Box, Button, TextField } from "@mui/material";
import { useStorageState } from "../../hooks";

export default function Settings() {
  const [value, setValue] = useStorageState("customCSS");

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
          setValue(e.target.value);
        }}
        placeholder="{ body: url('https://localhost' }"
        value={value}
      />
      <Button variant="outlined" onClick={() => setValue(sample)}>
        use Sample
      </Button>

      <style>{value}</style>
    </Box>
  );
}

const sample = `body {
    background-image: url("https://i.redd.it/red-forest-1920-1080-v0-s9u8ki2rr70a1.jpg?s=139edf608c428656505a143635a0687dec086229");
}

main > form > div > div,
main > div {
    background-color: #2424248c;
}
`;
