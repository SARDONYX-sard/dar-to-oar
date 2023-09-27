import * as React from "react";
import LoadingButton from "@mui/lab/LoadingButton";
import ConvertIcon from "@mui/icons-material/Transform";

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
  React.useEffect(() => {}, [loading, setLoading]);

  return (
    <LoadingButton
      type="submit"
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
