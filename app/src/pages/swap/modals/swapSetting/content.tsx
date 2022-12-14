import React, { FC, useCallback, ReactNode } from "react";
import { makeStyles } from "@mui/styles";
import { Box, Typography, Button, InputAdornment } from "@mui/material";
import cn from "classnames";
import NumericField from "../../../../components/numericField";

const useStyles = makeStyles({
  contentTitle: {
    borderBottom: "1px solid #FFFFFF0F",
    color: "#FFF",
    fontSize: "18px !important",
    fontWeight: "500 !important",
    lineHeight: "22px !important",
    padding: "23px 20px",
    margin: "0 3px",
  },
  contentWrapper: {
    padding: "32px 23px",
  },
  typography: {
    color: "#FFF",
    fontSize: "14px !important",
    lineHeight: "16px !important",
    opacity: "0.6",
  },
  optionWrapper: {
    display: "flex",
    marginTop: "13px",
    justifyContent: "space-between",
    "& button": {
      border: "1px solid #FFFFFF40",
      borderRadius: "4px",
      color: "#FFF",
      fontSize: "18px",
      fontWeight: "400",
      height: "48px",
      padding: "14px 6px !important",
      width: "70px",
      marginTop: "12px",
      marginRight: "12px",
      "&:last-of-type": {
        marginRight: "24px",
      },
    },
    "@media (max-width: 600px)": {
      flexWrap: "wrap",
      "& button": {
        marginRight: "0 !important",
        width: "30%",
      },
    },
  },
  optionActive: {
    background:
      "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%) !important",
    border: "none !important",
  },
  optionInput: {
    background: "#656D7A",
    borderRadius: "6px",
    marginTop: "12px !important",
    "& .MuiInputBase-root": {
      padding: "0 16px 0 0",
      height: "48px",
      "& fieldset": {
        display: "none",
      },
    },
    "& input": {
      color: "#FFF",
      fontSize: "18px",
      fontWeight: "400",
      padding: "14px 8px 14px 16px",
    },
    "& .MuiInputAdornment-root": {
      marginLeft: "4px",
      "& p": {
        color: "#FFF",
        fontSize: "18px",
        fontWeight: "400",
        lineHeight: "21px",
      },
    },
    "&::before": {
      content: "''",
      position: "absolute",
      top: "1px",
      right: "1px",
      bottom: "1px",
      left: "1px",
      background: "#313C4E",
      borderRadius: "6px",
    },
    "@media (max-width: 600px)": {
      order: "-1",
      width: "100%",
    },
  },
  inputError: {
    background: "#F74949",
  },
  inputActive: {
    background: "linear-gradient(88.14deg, #918EFF 16.49%, #19CE9D 86.39%)",
  },
  error: {
    color: "#F74949",
    fontSize: "14px !important",
    lineHeight: "16px !important",
    marginTop: "16px !important",
  },
});

interface ContentProps {
  slippage: bigint;
  setSlippage(value: bigint): void;
}

function SlippageButton({
  children,
  className,
  amount,
  onClick,
}: {
  amount: bigint;
  className?: string;
  children: ReactNode;
  onClick?: (amount: bigint) => void;
}) {
  const handleClick = useCallback(() => {
    onClick && onClick(amount);
  }, [onClick, amount]);
  return (
    <Button className={className} onClick={handleClick}>
      {children}
    </Button>
  );
}

const Content: FC<ContentProps> = ({ slippage, setSlippage }) => {
  const classes = useStyles();

  const isError = slippage < 1n;
  const handleSlippageButtonClicked = useCallback(
    (amount: bigint) => {
      setSlippage(amount);
    },
    [setSlippage]
  );

  return (
    <>
      <Typography className={classes.contentTitle}>Settings</Typography>
      <Box className={classes.contentWrapper}>
        <Typography className={classes.typography}>
          Slippage Tolerance
        </Typography>
        <Box className={classes.optionWrapper}>
          <SlippageButton
            className={cn({
              [classes.optionActive]: slippage === 10n,
            })}
            amount={10n}
            onClick={handleSlippageButtonClicked}
          >
            0.1%
          </SlippageButton>
          <SlippageButton
            className={cn({
              [classes.optionActive]: slippage === 50n,
            })}
            amount={50n}
            onClick={handleSlippageButtonClicked}
          >
            0.5%
          </SlippageButton>
          <SlippageButton
            className={cn({
              [classes.optionActive]: slippage === 100n,
            })}
            amount={100n}
            onClick={handleSlippageButtonClicked}
          >
            1.0%
          </SlippageButton>
          <NumericField
            className={cn(classes.optionInput, {
              [classes.inputError]: isError,
              [classes.inputActive]: !isError,
            })}
            hiddenLabel
            InputProps={{
              endAdornment: <InputAdornment position="end">%</InputAdornment>,
            }}
            value={Number(slippage) / 100}
            onChange={(value) => setSlippage(BigInt(value * 100))}
          />
        </Box>
        {isError && (
          <Typography className={classes.error}>
            Enter a valid slippage percentage
          </Typography>
        )}
      </Box>
    </>
  );
};

export default Content;
