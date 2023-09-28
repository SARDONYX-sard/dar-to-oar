"use client";

import { Toaster } from "react-hot-toast";
import { useToastLimit } from "@/hooks";
import { ConvertForm } from "@/components/form";
import { Box } from "@mui/material";
import { useStorageState } from "../../hooks";

export default function Converter() {
  useToastLimit(1);
  const [value, _setValue] = useStorageState("customCSS");

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
      <style>{value}</style>
      <Toaster position="bottom-right" reverseOrder={false} />
    </>
  );
}
