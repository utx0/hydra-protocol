import React, { FC, useState, useEffect } from "react";
import { Box, Menu, IconButton, Typography } from "@mui/material";
import { makeStyles } from "@mui/styles";
import cn from "classnames";

import { CaretDown, Wallet as WalletSVG, User } from "../../icons";
import HYSD from "../../../assets/images/symbols/hysd.png";
import { useAssetBalances, useWallet } from "hydra-react-ts";
import { toFormat } from "../../../utils/toFormat";
import { normalizeAddress } from "../../../helpers/normalize";
import { Asset } from "../../../types";

const useStyles = makeStyles({
  connectButton: {
    background: "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%)",
    borderRadius: "6px !important",
    display: "flex",
    alignItems: "center",
    padding: "12px 20px !important",
    "& > svg": {
      fill: "#FFF",
      width: "16px !important",
      height: "16px !important",
      marginRight: "8px",
    },
    "& > p": {
      color: "#FFF",
    },
    "@media (max-width: 600px)": {
      padding: "8px 20px !important",
      "& > p": {
        fontSize: "14px !important",
        lineHeight: "20px !important",
      },
    },
  },
  tokensButton: {
    backgroundColor: "#3F414C !important",
    borderRadius: "6px !important",
    padding: "14px 12px !important",
    "& > svg": {
      position: "relative",
      "&:first-of-type": {
        fill: "#FFFFFFD9 !important",
        width: "21px",
        height: "23px",
      },
      "&:last-of-type": {
        fill: "#FFFFFF73",
        width: "9px",
        height: "6px",
      },
    },
    "& > p": {
      position: "relative",
      color: "#FFFFFFD9 !important",
      lineHeight: "19px !important",
      marginLeft: "17px",
      marginRight: "15px",
    },
    "&::before": {
      content: "''",
      position: "absolute",
      top: "1px",
      right: "1px",
      bottom: "1px",
      left: "1px",
      borderRadius: "6px",
      background: "#1D202D",
    },
    "&:hover": {
      background:
        "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%) !important",
    },
    "@media (max-width: 600px)": {
      padding: "7px 8px !important",
      "& > svg": {
        width: "16px !important",
        height: "18px !important",
      },
      "& > p": {
        fontSize: "14px !important",
        lineHeight: "20px !important",
      },
    },
  },
  activeTokensButton: {
    background:
      "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%) !important",
  },
  walletButton: {
    backgroundColor: "#3F414C !important",
    borderRadius: "6px !important",
    padding: "12px !important",
    marginRight: "8px !important",
    "& > svg": {
      fill: "#FFFFFFD9",
      position: "relative",
    },
    "&::before": {
      content: "''",
      position: "absolute",
      top: "1px",
      right: "1px",
      bottom: "1px",
      left: "1px",
      borderRadius: "6px",
      background: "#1D202D",
    },
    "&:hover": {
      background:
        "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%) !important",
    },
    "@media (max-width: 600px)": {
      padding: "8px 7px !important",
      marginRight: "16px !important",
      "& > svg": {
        width: "18px !important",
        height: "16px !important",
      },
    },
  },
  tokensWrapper: {
    "& .MuiPaper-root": {
      backgroundColor: "initial !important",
      marginTop: "4px",
    },
    "& .MuiList-root": {
      background:
        "linear-gradient(180deg, rgba(41, 255, 200, 0.25) 0%, rgba(1, 207, 237, 0) 100%)",
      borderRadius: "6px",
      padding: "1px",
    },
  },
  tokensContent: {
    background: "#2d3444",
    borderRadius: "5px",
    width: "300px",
    "& > p": {
      borderBottom: "1px solid #FFFFFF0A",
      color: "#FFFFFFD9",
      fontSize: "18px !important",
      lineHeight: "21px !important",
      padding: "15px 23px",
    },
  },
  tokensList: {
    padding: "0 23px",
    maxHeight: "300px",
    overflowY: "auto",
    "@media (max-width: 600px)": {
      maxHeight: "600px",
    },
  },
  tokenItem: {
    borderBottom: "1px solid #FFFFFF0A",
    display: "flex",
    alignItems: "center",
    justifyContent: "flex-start",
    padding: "14px 0",
    "& > img": {
      width: "32px",
      height: "32px",
      marginRight: "10px",
    },
    "& > div": {
      "& > p": {
        "&:first-of-type": {
          color: "#FFF",
          fontSize: "14px !important",
          fontWeight: "500 !important",
          lineHeight: "17px !important",
          marginBottom: "4px",
        },
        // "&:last-of-type": {
        //   color: "#FFFFFFA6",
        //   fontSize: "12px !important",
        //   lineHeight: "14px !important",
        // },
      },
    },
    "&:last-of-type": {
      borderBottom: "none",
    },
  },
  tokenImgWrapper: {
    display: "flex",
    alignItmes: "center",
    justifyContent: "center",
    width: "32px",
    height: "32px",
    marginRight: "10px",
    "& > img": {
      maxWidth: "100%",
      maxHeight: "100%",
    },
  },
});

interface WalletButtonProps {
  openWalletModal(): void;
  updateAddress?(address: string): void;
}

const WalletButton: FC<WalletButtonProps> = ({
  openWalletModal,
  updateAddress,
}) => {
  const classes = useStyles();

  const balances = useAssetBalances();

  const { connected, publicKey } = useWallet();
  const [address, setAddress] = useState("");
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const tokenListOpen = Boolean(anchorEl);
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    // Windows Resize Handler
    function handleResize() {
      setIsMobile(window.innerWidth <= 600);
    }

    // Add event listener
    window.addEventListener("resize", handleResize);

    handleResize();

    // Remove event listener on cleanup
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  useEffect(() => {
    if (connected) {
      const base58 = publicKey ? publicKey.toBase58() : "";
      setAddress(base58);

      if (updateAddress) {
        updateAddress(base58);
      }
    }
  }, [connected, publicKey, updateAddress]);

  const handleOpenTokenList = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleCloseTokenList = () => {
    setAnchorEl(null);
  };

  return (
    <>
      {connected && (
        <>
          <IconButton
            className={classes.walletButton}
            onClick={openWalletModal}
          >
            {<WalletSVG />}
          </IconButton>
          <IconButton
            className={cn(classes.tokensButton, {
              [classes.activeTokensButton]: tokenListOpen,
            })}
            onClick={handleOpenTokenList}
          >
            <User />
            {!isMobile && (
              <>
                <Typography>{normalizeAddress(address)}</Typography>
                <CaretDown />
              </>
            )}
          </IconButton>
          <Menu
            className={classes.tokensWrapper}
            anchorEl={anchorEl}
            open={tokenListOpen}
            onClose={handleCloseTokenList}
            anchorOrigin={{
              vertical: "bottom",
              horizontal: "right",
            }}
            transformOrigin={{
              vertical: "top",
              horizontal: "right",
            }}
          >
            <Box className={classes.tokensContent}>
              <Typography>Your Tokens</Typography>
              <Box className={classes.tokensList}>
                {balances.map((balance: Asset) => (
                  <Box className={classes.tokenItem} key={balance.symbol}>
                    <span className={classes.tokenImgWrapper}>
                      <img
                        src={
                          balance.symbol.includes("HYD")
                            ? HYSD
                            : balance.logoURI
                        }
                        alt="Token"
                      />
                    </span>
                    <Box>
                      <Typography>
                        {toFormat(balance.balance || 0n, balance.decimals)}{" "}
                        {balance.symbol}
                      </Typography>
                      {/* <Typography>$</Typography> */}
                    </Box>
                  </Box>
                ))}
              </Box>
            </Box>
          </Menu>
        </>
      )}
      {!connected && (
        <IconButton className={classes.connectButton} onClick={openWalletModal}>
          <WalletSVG />
          <Typography>{isMobile ? "Connect" : "Connect Wallet"}</Typography>
        </IconButton>
      )}
    </>
  );
};

export default WalletButton;
