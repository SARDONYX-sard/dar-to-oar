import { Tooltip } from "@mui/material";
import Button from "@mui/material/Button";
import { restoreDarDir } from "@/tauri_cmd";
import toast from "react-hot-toast";
import RestoreIcon from "@mui/icons-material/Restore";

type Props = {
  path: string;
};

export const RestoreDarBtn = ({ path }: Props) => {
  return (
    <Tooltip
      title={
        <p>
          Restore the directory hidden by &quot;Hide DAR&quot;.(For MO2 user)
        </p>
      }
    >
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
        startIcon={<RestoreIcon />}
      >
        Restore DAR
      </Button>
    </Tooltip>
  );
};
