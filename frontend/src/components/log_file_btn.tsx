import { Button } from "@mui/material";
import { openLogFile } from "@/tauri_cmd";
import toast from "react-hot-toast";

export const LogFileButton = () => {
  return (
    <Button
      type="button"
      sx={{
        marginTop: "9px",
        width: "100%",
        height: "80%",
      }}
      variant="outlined"
      onClick={async () => openLogFile().catch((e) => toast.error(`${e}`))}
    >
      Open log
    </Button>
  );
};
