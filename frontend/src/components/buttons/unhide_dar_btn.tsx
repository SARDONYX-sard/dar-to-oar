import { Tooltip } from "@mui/material";
import Button from "@mui/material/Button";
import { restoreDarDir } from "@/tauri_cmd";
import toast from "react-hot-toast";
import VisibilityIcon from "@mui/icons-material/Visibility";
import { useTranslation } from "react-i18next";

type Props = {
  path: string;
};


  export const UnhideDarBtn = ({ path }: Props) => {
    const { t } = useTranslation();

  return (
    <Tooltip title={<p>{t("unhide-dar-btn-tooltip")}</p>}>
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
            toast.success(await restoreDarDir(path));
          } catch (err) {
            toast.error(`${err}`);
          }
        }}
        startIcon={<VisibilityIcon />}
      >
        {t("unhide-dar-btn-name")}
      </Button>
    </Tooltip>
  );
};
