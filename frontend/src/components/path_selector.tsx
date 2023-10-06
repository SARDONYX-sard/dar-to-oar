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
    <Button type="button" onClick={handleClick}>
      Select
    </Button>
  );
}
