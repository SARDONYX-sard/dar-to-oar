import * as React from "react";
import ConvertIcon from "@mui/icons-material/Transform";
import LoadingButton from "@mui/lab/LoadingButton";

type Props = {
  loading: boolean;
  setLoading: (loading: boolean) => void;
};

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export default function ConvertButton({ loading, setLoading }: Props) {
  return (
    <LoadingButton
      type="submit"
      sx={{ width: "100%" }}
      endIcon={<ConvertIcon />}
      loading={loading}
      loadingPosition="end"
      variant="contained"
      onChange={() => setLoading(true)}
    >
      <span>{loading ? "Converting..." : "Convert"}</span>
    </LoadingButton>
  );
}
