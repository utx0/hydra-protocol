import {
  Button,
  Dialog,
  DialogTitle,
  DialogActions,
  useMediaQuery,
} from "@mui/material";
import { useTheme } from "@mui/material/styles";
import { toFormat } from "../utils/toFormat";
import { Asset } from "../types";

export function SwapPreviewModal({
  open,
  handleClose,
  fromAsset,
  fromAmount,
  toAmount,
  toAsset,
  handleSubmit,
}: {
  fromAsset: Asset;
  fromAmount: bigint;
  toAsset: Asset;
  toAmount: bigint;
  open: boolean;
  handleClose: () => void;
  handleSubmit: () => void;
}) {
  const theme = useTheme();
  const fullScreen = useMediaQuery(theme.breakpoints.down("md"));

  return (
    <Dialog
      fullScreen={fullScreen}
      open={open}
      onClose={handleClose}
      aria-labelledby="responsive-dialog-title"
    >
      <DialogTitle id="responsive-dialog-title">
        Swap {toFormat(fromAmount, fromAsset.decimals)} {fromAsset.name} to{" "}
        {toFormat(toAmount, toAsset.decimals)} {toAsset.name}
      </DialogTitle>

      <DialogActions>
        <Button autoFocus onClick={handleClose}>
          Cancel
        </Button>
        <Button onClick={handleSubmit} autoFocus>
          Swap
        </Button>
      </DialogActions>
    </Dialog>
  );
}
