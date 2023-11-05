import Button from "@mui/material/Button";
import DeleteIcon from "@mui/icons-material/Delete";
import Tooltip from "@mui/material/Tooltip";
import toast from "react-hot-toast";
import { removeOarDir } from "@/tauri_cmd";
import { useTranslation } from "react-i18next";

type Props = {
  darPath: string;
  oarPath: string;
};

export const RemoveOarBtn = ({ darPath, oarPath }: Props) => {
  const { t } = useTranslation();
  return (
    <Tooltip title={<p>{t("remove-oar-tooltip")}</p>}>
      <Button
        type="button"
        sx={{
          marginTop: "9px",
          width: "100%",
          height: "60%",
        }}
        variant="outlined"
        onClick={async () => {
          try {
            await removeOarDir(darPath);
            toast.success(t("remove-oar-success"));
          } catch (_) {
            try {
              await removeOarDir(oarPath);
              toast.success(t("remove-oar-success"));
            } catch (e) {
              toast.error(`${e}`);
            }
          }
        }}
        startIcon={<DeleteIcon />}
      >
        {t("remove-oar-btn")}
      </Button>
    </Tooltip>
  );
};
