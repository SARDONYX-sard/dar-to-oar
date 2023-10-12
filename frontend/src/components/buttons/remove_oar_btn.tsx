import { removeOarDir } from "@/tauri_cmd";
import toast from "react-hot-toast";
import DeleteIcon from "@mui/icons-material/Delete";
import Button from "@mui/material/Button";
import Tooltip from "@mui/material/Tooltip";

type Props = {
  darPath: string;
  oarPath: string;
};

export const RemoveOarBtn = ({ darPath, oarPath }: Props) => {
  return (
    <Tooltip
      title={
        <p>
          Find and delete OAR dir from &quot;DAR(src) Directory*&quot; or
          &quot;OAR(dist) Directory&quot;.
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
            await removeOarDir(darPath);
            toast.success("Removed OAR directory.");
          } catch (_) {
            try {
              await removeOarDir(oarPath);
              toast.success("Removed OAR directory.");
            } catch (e) {
              toast.error(`${e}`);
            }
          }
        }}
        startIcon={<DeleteIcon />}
      >
        Remove OAR
      </Button>
    </Tooltip>
  );
};
