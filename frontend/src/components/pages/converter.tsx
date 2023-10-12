"use client";

import { ConvertForm } from "@/components/form";
import { useDynStyle, useInjectScript, useToastLimit } from "@/hooks";
import { Box } from "@mui/material";
import { Toaster } from "react-hot-toast";

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
          placeItems: "center",
          placeContent: "center",
          height: "calc(100% - 56px)",
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
            }
        }}
      />
    </>
  );
}
