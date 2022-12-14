import {
  Button,
  Dialog,
  DialogTitle,
  DialogActions,
  useMediaQuery,
} from "@mui/material";
import { useTheme } from "@mui/material/styles";
import { Asset } from "../../types";
import { Box } from "@mui/system";

export function RemoveLiquidityPreviewModal({
  open,
  handleClose,
  tokenAAsset,
  percent,
  tokenBAsset,
  handleSubmit,
}: {
  tokenAAsset: Asset;
  tokenBAsset: Asset;
  percent: bigint;
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
        <Box>Remove liquidity</Box>
        <Box>
          Percent:
          {percent} {tokenAAsset.symbol} / {tokenBAsset.symbol}
        </Box>
      </DialogTitle>

      <DialogActions>
        <Button autoFocus onClick={handleClose}>
          Cancel
        </Button>
        <Button onClick={handleSubmit} autoFocus>
          Add Liquidity
        </Button>
      </DialogActions>
    </Dialog>
  );
}
