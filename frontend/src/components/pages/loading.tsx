"use client";

import { Box, CircularProgress } from "@mui/material";
import { useDynStyle } from "../../hooks";

export default function Loading() {
  useDynStyle();
  return (
    <>
      <Box
        sx={{
          display: "grid",
          placeContent: "center",
          placeItems: "center",
          height: "100vh",
        }}
      >
        <h1>Loading...</h1>
        <CircularProgress />
      </Box>
    </>
  );
}
