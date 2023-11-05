"use client";

import { Box } from "@mui/material";
import { ConvertForm } from "@/components/form";
import { Toaster } from "react-hot-toast";
import { useDynStyle, useInjectScript, useToastLimit } from "@/hooks";

export default function Converter() {
  useToastLimit(1);
  useDynStyle();
  useInjectScript();

  return (
    <>
      <Box
        component="main"
        sx={{
          display: "grid",
          minHeight: "calc(100vh - 56px)",
          placeContent: "center",
          placeItems: "center",
          width: "100%",
        }}
      >
        <ConvertForm />
      </Box>
      <Toaster
        position="bottom-right"
        reverseOrder={false}
        toastOptions={{
          style: {
            color: "#fff",
            background: "#1a1919e1",
          },
        }}
      />
    </>
  );
}
