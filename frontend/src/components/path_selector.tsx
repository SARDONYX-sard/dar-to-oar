import { Button } from "@mui/material";
import toast from "react-hot-toast";
import { openPath } from "../tauri_cmd";

type Props = {
  path: string;
  isDir?: boolean;
  setValue: (value: string) => void;
};

export function PathSelector({ path, isDir = false, setValue }: Props) {
  const handleClick = async () => {
    openPath(path, setValue, isDir).catch((e) => toast.error(`${e}`));
  };

  return (
    <Button
      sx={{
        marginTop: "9px",
        width: "100%",
        height: "55px",
      }}
      variant="outlined"
      type="button"
      onClick={handleClick}
    >
      Select
    </Button>
  );
}
