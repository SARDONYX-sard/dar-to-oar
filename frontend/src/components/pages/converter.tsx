"use client";

import { Toaster } from "react-hot-toast";
import { useDynStyle, useStorageState, useToastLimit } from "@/hooks";
import { ConvertForm } from "@/components/form";
import { Box } from "@mui/material";

export default function Converter() {
  useToastLimit(1);
  const [style, _setStyle] = useStorageState("customCSS");
  useDynStyle(style);

  return (
    <>
      <Box
        component="main"
        sx={{
          display: "grid",
          placeItems: "center",
          placeContent: "center",
          height: "calc(100% - 56px)",
          width: "100%",
        }}
      >
        <ConvertForm />
      </Box>
      <Toaster position="bottom-right" reverseOrder={false} />
    </>
  );
}
