import toast from "react-hot-toast";
import { Button } from "@mui/material";
import { FileOpen } from "@mui/icons-material";
import { openLogFile } from "@/tauri_cmd";
import { useTranslation } from "react-i18next";

export const LogFileButton = () => {
  const { t } = useTranslation();

  return (
    <Button
      sx={{
        marginTop: "9px",
        width: "100%",
        height: "60%",
      }}
      onClick={async () => openLogFile().catch((e) => toast.error(`${e}`))}
      startIcon={<FileOpen />}
      type="button"
      variant="outlined"
    >
      {t("open-log-btn")}
    </Button>
  );
};
