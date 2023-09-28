import { Box, CircularProgress } from "@mui/material";
export default function Loading() {
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
