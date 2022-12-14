import { FC, useState, useMemo } from "react";
import { makeStyles } from "@mui/styles";
import { Box, Typography } from "@mui/material";
import { useConnection, useAnchorWallet } from "@solana/wallet-adapter-react";
import { HydraSDK, Network } from "hydra-ts";
import { useObservable } from "react-use";
import { toast } from "react-toastify";

import { Deposit } from "../../components/icons";
import Banner from "../../assets/images/stake/banner.png";
import Diamond from "../../assets/images/stake/diamond.png";
import StakeUnstake from "./stakeUnstake";
import StakeStatus from "./stakeStatus";

const useStyles = makeStyles({
  stakeContainer: {
    display: "flex",
    flexDirection: "column",
    width: "100%",
    maxWidth: "1100px",
  },
  stakeBanner: {
    background: "#FFFFFF05",
    borderRadius: "6px",
    display: "flex",
    alignItems: "center",
    height: "200px",
    width: "100%",
    "@media (max-width: 600px)": {
      flexDirection: "column",
      alignItems: "flex-start",
      padding: "20px 0",
      height: "initial",
    },
  },
  bannerLeft: {
    backgroundImage: `url(${Banner})`,
    backgroundRepeat: "no-repeat",
    backgroundSize: "100% 100%",
    flexGrow: 1,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    position: "relative",
    height: "100%",
    "& > img": {
      width: "140px",
      "&:first-of-type": {
        marginLeft: "50px",
        transform: "rotate(135deg)",
      },
      "@media (max-width: 1350px)": {
        width: "120px",
      },
      "@media (max-width: 1100px)": {
        width: "90px",
      },
      "@media (max-width: 1000px)": {
        display: "none",
      },
    },
    "@media (max-width: 600px)": {
      background: "none",
      display: "block",
    },
  },
  bannerTitle: {
    padding: "0 24px",
    "& p": {
      "&:first-of-type": {
        color: "#19CE9D",
        fontSize: "32px",
        fontWeight: "600 !important",
        lineHeight: "24px",
        marginBottom: "16px",
      },
      "&:last-of-type": {
        color: "#FFF",
      },
    },
    "@media (max-width: 1000px)": {
      padding: "0 48px 0 24px",
    },
    "@media (max-width: 600px)": {
      padding: "0 24px",
      "& p": {
        "&:first-of-type": {
          lineHeight: "39px !important",
        },
      },
    },
  },
  bannerRight: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    padding: "0 60px",
    "& svg": {
      width: "32px !important",
      height: "32px !important",
      marginBottom: "11px",
    },
    "& p": {
      lineHeight: "24px !important",
      "&:first-of-type": {
        color: "#FFFFFFA6",
        fontSize: "14px !important",
        marginBottom: "6px !important",
      },
      "&:last-of-type": {
        color: "#FFF",
        fontSize: "32px !important",
        fontWeight: "500 !important",
      },
    },
    "@media (max-width: 1350px)": {
      padding: "0 40px",
      minWidth: "137px",
    },
    "@media (max-width: 1000px)": {
      padding: "0 24px",
    },
    "@media (max-width: 600px)": {
      alignItems: "flex-start",
      marginTop: "24px",
      "& > svg": {
        display: "none",
      },
    },
  },
  stakeContent: {
    display: "flex",
    marginTop: "24px",
    "& > div": {
      height: "100%",
      "&:first-of-type": {
        width: "40%",
      },
      "&:last-of-type": {
        marginLeft: "24px",
        width: "calc(60% - 24px)",
      },
    },
    "@media (max-width: 1100px)": {
      flexDirection: "column",
      "& > div": {
        width: "calc(100% - 4px) !important",
        "&:last-of-type": {
          marginLeft: "0 !important",
          marginTop: "24px",
        },
      },
    },
  },
});

interface StakeProps {
  openWalletConnect(): void;
}

const Stake: FC<StakeProps> = ({ openWalletConnect }) => {
  const classes = useStyles();
  const wallet = useAnchorWallet();
  const { connection } = useConnection();

  const [staking, setStaking] = useState(false);
  const [unstaking, setUnstaking] = useState(false);

  // TODO: Replace with useHydraClient()
  const sdk = useMemo(
    () => HydraSDK.create(Network.LOCALNET, connection, wallet),
    [connection, wallet]
  );

  const userFrom = useObservable(
    useMemo(() => sdk.staking.accounts.userToken.stream(), [sdk])
  );

  const userRedeemable = useObservable(
    useMemo(() => sdk.staking.accounts.userRedeemable.stream(), [sdk])
  );

  const tokenVault = useObservable(
    useMemo(() => sdk.staking.accounts.tokenVault.stream(), [sdk])
  );

  const stake = async (amount: string) => {
    setStaking(true);

    try {
      await sdk.staking.stake(BigInt(amount));
      toast.success(`You staked ${amount} HYSD successfully.`);
    } catch (error) {
      console.log(error);
      toast.error(`Staking ${amount} HYSD failed.`);
    }

    setStaking(false);
  };

  const unstake = async (amount: string) => {
    setUnstaking(true);

    try {
      await sdk.staking.unstake(BigInt(amount));
      toast.success(`You unstaked ${amount} HYSD successfully.`);
    } catch (error) {
      console.log(error);
      toast.error(`Unstaking ${amount} HYSD failed.`);
    }

    setUnstaking(false);
  };

  return (
    <Box className={classes.stakeContainer}>
      <Box className={classes.stakeBanner}>
        <Box className={classes.bannerLeft}>
          <img src={Diamond} alt="Diamond" />
          <Box className={classes.bannerTitle}>
            <Typography>Simply stake tokens to earn.</Typography>
            <Typography>
              Stake your HYSD maximize your yield. No Impermanent Loss.
            </Typography>
          </Box>
          <img src={Diamond} alt="Diamond" />
        </Box>
        <Box className={classes.bannerRight}>
          <Deposit />
          <Typography>Total Staked</Typography>
          <Typography>$12.56 m</Typography>
        </Box>
      </Box>
      <Box className={classes.stakeContent}>
        <StakeUnstake
          walletConnect={openWalletConnect}
          balance={userFrom ? `${userFrom.account.data.amount}` : "0"}
          xBalance={
            userRedeemable ? `${userRedeemable.account.data.amount}` : "0"
          }
          onStake={stake}
          onUnstake={unstake}
          staking={staking}
          unstaking={unstaking}
        />
        <StakeStatus
          balance={tokenVault ? `${tokenVault.account.data.amount}` : "0"}
        />
      </Box>
    </Box>
  );
};

export default Stake;
