"use client";

import { ConvertForm } from "@/components/form";
import { useDynStyleWithStorage, useToastLimit } from "@/hooks";
import { Box } from "@mui/material";
import { Toaster } from "react-hot-toast";

export default function Converter() {
  useToastLimit(1);
  useDynStyleWithStorage();

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
