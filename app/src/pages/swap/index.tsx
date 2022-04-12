import { FC, useState, useEffect } from "react";
import { makeStyles } from "@mui/styles";
import { Box, IconButton } from "@mui/material";

import { Gear } from "../../components/icons";
import { Asset, AssetBalance } from "../../types";
import SwapAsset from "./swapAsset";
import SwapSettingModal from "./modals/swapSetting";
import AssetListModal from "./modals/assetList";
import ConfirmSwapModal from "./modals/confirmSwap";
import SwapStatus from "./modals/swapStatus";
import { useSwap } from "./hooks/useSwap";
import { useAssetBalances } from "./hooks/useAssetBalances";
import { toFormat } from "../../utils/toFormat";

const useStyles = makeStyles({
  swapContent: {
    display: "flex",
    flexDirection: "column",
    maxWidth: "450px",
    width: "100%",
  },
  actionRow: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: "16px",
  },
  title: {
    color: "#FFF",
    fontSize: "20px",
    fontWeight: "500",
    lineHeight: "24px",
  },
  swapIcons: {
    display: "flex",
    alignItems: "center",
    "& .MuiSvgIcon-root": {
      width: "20px",
      height: "20px",
      color: "#FFFFFFA6",
    },
    "& .MuiIconButton-root": {
      padding: "0",
    },
  },
  swapAssets: {
    background:
      "linear-gradient(180deg, rgba(41, 255, 200, 0.25) 0%, rgba(1, 207, 237, 0) 100%)",
    borderRadius: "6px",
    display: "flex",
    flexDirection: "column",
    padding: "2px",
  },
  routeContainer: {
    background: "#262936",
    borderRadius: "6px",
    padding: "20px 32px",
    margin: "2px",
  },
  routeTitle: {
    color: "#FFFFFFA6 !important",
    fontSize: "14px !important",
    lineHeight: "17px !important",
    textDecoration: "underline",
    marginBottom: "16px !important",
  },
  routeDetail: {
    display: "flex",
  },
  assetItem: {
    color: "#FFFFFFD9",
    display: "flex",
    alignItems: "center",
    marginRight: "12px",
    "& img": {
      marginRight: "4px",
      width: "20px",
      height: "20px",
    },
    "& span": {
      fontSize: "14px",
      lineHeight: "17px",
    },
    "&:last-of-type": {
      marginRight: "0",
    },
  },
});

interface SwapProps {
  openWalletConnect(): void;
}

const Swap: FC<SwapProps> = ({ openWalletConnect }) => {
  const classes = useStyles();

  const {
    tokenFrom,
    tokenTo,
    assetsTokenFrom,
    assetsTokenTo,
    toggleFields,
    poolExists,
    poolPairSelected,
    canSwap,
    setFocus,
    onSendSubmit,
    state,
    onSendCancel,
  } = useSwap();
  const balances = useAssetBalances();

  // const [swapRate, setSwapRate] = useState(0);
  const [activeAsset, setActiveAsset] = useState("");
  const [assetList, setAssetList] = useState<Asset[]>([]);
  const [assetsBalance, setAssetsBalance] = useState<AssetBalance>({});
  const [slippage, setSlippage] = useState("1.0");
  const [openSettingModal, setOpenSettingModal] = useState(false);
  const [openAssetListModal, setOpenAssetListModal] = useState(false);
  const [openConfirmSwapModal, setOpenConfirmSwapModal] = useState(false);
  const [openSwapStatusModal, setOpenSwapStatusModal] = useState(false);

  useEffect(() => {
    let tempBalances: AssetBalance = {};

    balances.forEach((balance) => {
      let tempBalance = balance.balance || 0n;
      tempBalances[balance["address"]] = toFormat(
        tempBalance,
        balance.decimals
      );
    });

    setAssetsBalance(tempBalances);
  }, [balances]);

  const handleSettingModal = () => {
    if (parseFloat(slippage) > 0) setOpenSettingModal(false);
  };

  const handleChangeAsset = (type: string) => {
    setActiveAsset(type);
    setAssetList(type === "From" ? assetsTokenFrom : assetsTokenTo);
    setOpenAssetListModal(true);
  };

  const changeAsset = (asset: Asset) => {
    if (activeAsset === "From") tokenFrom.setAsset(asset);
    else tokenTo.setAsset(asset);

    setActiveAsset("");
    setOpenAssetListModal(false);
  };

  const handleOpenConfirmSwap = () => {
    onSendSubmit();
    setOpenConfirmSwapModal(true);
  };

  const handleCloseConfirmSwap = () => {
    onSendCancel();
    setOpenConfirmSwapModal(false);
  };

  const handleSwapApprove = () => {
    onSendSubmit();
    setOpenConfirmSwapModal(false);
    setOpenSwapStatusModal(true);
  };

  const handleSwapStatusClose = () => {
    onSendCancel();
    setOpenSwapStatusModal(false);
  };

  return (
    <>
      <Box className={classes.swapContent}>
        <Box className={classes.actionRow}>
          <span className={classes.title}>Swap</span>
          <div className={classes.swapIcons}>
            <IconButton onClick={() => setOpenSettingModal(true)}>
              <Gear />
            </IconButton>
          </div>
        </Box>
        <Box className={classes.swapAssets}>
          <SwapAsset
            fromAsset={tokenFrom}
            toAsset={tokenTo}
            changeAsset={handleChangeAsset}
            balances={assetsBalance}
            assetFocus={setFocus}
            exchangeAsset={toggleFields}
            canSwap={canSwap}
            poolExits={poolExists}
            poolPairSelected={poolPairSelected}
            confirmSwap={handleOpenConfirmSwap}
            walletConnect={openWalletConnect}
          />
        </Box>
        {/* {fromAsset.symbol !== "" && toAsset.symbol !== "" && (
          <Box className={classes.routeContainer}>
            <Typography className={classes.routeTitle}>Route</Typography>
            <Box className={classes.routeDetail}>
              <Box className={classes.assetItem}>
                <img src={fromAsset.icon} alt="Asset" />
                <span>{`${fromAsset.symbol} >`}</span>
              </Box>
              <Box className={classes.assetItem}>
                <img src={USDT} alt="Asset" />
                <span>{"USDT >"}</span>
              </Box>
              <Box className={classes.assetItem}>
                <img src={toAsset.icon} alt="Asset" />
                <span>{toAsset.symbol}</span>
              </Box>
            </Box>
          </Box>
        )} */}
      </Box>
      <SwapSettingModal
        open={openSettingModal}
        onClose={() => handleSettingModal()}
        slippage={slippage}
        setSlippage={(value) => setSlippage(value)}
      />
      <AssetListModal
        open={openAssetListModal}
        onClose={() => setOpenAssetListModal(false)}
        assetList={assetList}
        setAsset={changeAsset}
        balances={assetsBalance}
      />
      {tokenFrom.asset && tokenTo.asset && (
        <>
          <ConfirmSwapModal
            open={openConfirmSwapModal}
            onClose={handleCloseConfirmSwap}
            fromAsset={tokenFrom.asset}
            fromAmount={tokenFrom.amount}
            toAsset={tokenTo.asset}
            toAmount={tokenTo.amount}
            onApprove={handleSwapApprove}
          />
          <SwapStatus
            open={openSwapStatusModal}
            onClose={handleSwapStatusClose}
            fromAsset={tokenFrom.asset}
            fromAmount={tokenFrom.amount}
            toAsset={tokenTo.asset}
            toAmount={tokenTo.amount}
            state={state.value}
          />
        </>
      )}
    </>
  );
};

export default Swap;
