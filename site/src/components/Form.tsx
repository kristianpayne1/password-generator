import { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import FormControl from "@mui/material/FormControl";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";
import MuiCard from "@mui/material/Card";
import { styled } from "@mui/material/styles";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Button,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
} from "@mui/material";
import * as wasm from "password-generator";
import {
  ContentCopy as ContentCopyIcon,
  ExpandMore as ExpandMoreIcon,
  Loop as LoopIcon,
} from "@mui/icons-material";
import { Options } from "./Options.tsx";
import LinearProgressWithLabel from "./LinearProgressWithLabel.tsx";

const Card = styled(MuiCard)(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  alignSelf: "center",
  width: "100%",
  padding: theme.spacing(4),
  gap: theme.spacing(2),
  margin: "auto",
  [theme.breakpoints.up("sm")]: {
    maxWidth: "450px",
  },
  boxShadow:
    "hsla(220, 30%, 5%, 0.05) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.05) 0px 15px 35px -5px",
  ...theme.applyStyles("dark", {
    boxShadow:
      "hsla(220, 30%, 5%, 0.5) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.08) 0px 15px 35px -5px",
  }),
}));

const Container = styled(Stack)(({ theme }) => ({
  height: "calc((1 - var(--template-frame-height, 0)) * 100dvh)",
  minHeight: "100%",
  padding: theme.spacing(2),
  [theme.breakpoints.up("sm")]: {
    padding: theme.spacing(4),
  },
  "&::before": {
    content: '""',
    display: "block",
    position: "absolute",
    zIndex: -1,
    inset: 0,
    backgroundImage:
      "radial-gradient(ellipse at 50% 50%, hsl(210, 100%, 97%), hsl(0, 0%, 100%))",
    backgroundRepeat: "no-repeat",
    ...theme.applyStyles("dark", {
      backgroundImage:
        "radial-gradient(at 50% 50%, hsla(210, 100%, 16%, 0.5), hsl(220, 30%, 5%))",
    }),
  },
}));

function getPasswordStrengthMessageStatus(passwordStrength: number): {
  text: string;
  color: "error" | "warning" | "success";
} {
  if (passwordStrength <= 40) return { text: "Weak", color: "error" };
  if (passwordStrength <= 50) return { text: "OK", color: "warning" };
  return { text: "Strong", color: "success" };
}

export function Form() {
  const [password, setPassword] = useState("");
  const [length, setLength] = useState(12);
  const [includeNumber, setIncludeNumber] = useState(true);
  const [includeUppercase, setIncludeUppercase] = useState(true);
  const [includeLowercase, setIncludeLowercase] = useState(true);
  const [includeSymbols, setIncludeSymbols] = useState(true);

  const generatePassword = () =>
    wasm.generate(
      length,
      includeNumber,
      includeUppercase,
      includeLowercase,
      includeSymbols,
    );

  useEffect(() => {
    if (
      !includeNumber &&
      !includeUppercase &&
      !includeLowercase &&
      !includeSymbols
    )
      return setIncludeLowercase(true);

    setPassword(generatePassword());
  }, [
    length,
    includeNumber,
    includeUppercase,
    includeLowercase,
    includeSymbols,
  ]);

  const passwordStrength = wasm.check_strength(password);
  const { text, color } = getPasswordStrengthMessageStatus(passwordStrength);

  return (
    <>
      <CssBaseline enableColorScheme />
      <Container direction="column" justifyContent="space-between">
        <Card variant="outlined">
          <Typography
            component="h1"
            variant="h4"
            sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
          >
            Password Generator
          </Typography>
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              width: "100%",
              gap: 2,
            }}
          >
            <FormControl>
              <InputLabel htmlFor="generated-password">Password</InputLabel>
              <OutlinedInput
                id="generated-password"
                value={password}
                label=" Password"
                endAdornment={
                  <InputAdornment position="end">
                    <IconButton
                      aria-label={"Copy to clipboard"}
                      edge="end"
                      onClick={() => navigator.clipboard.writeText(password)}
                    >
                      <ContentCopyIcon />
                    </IconButton>
                  </InputAdornment>
                }
              />
            </FormControl>
          </Box>
          <LinearProgressWithLabel
            variant="determinate"
            value={passwordStrength}
            color={color}
            label={text}
          />
          <Accordion>
            <AccordionSummary expandIcon={<ExpandMoreIcon />}>
              <Typography component="span">More options</Typography>
            </AccordionSummary>
            <AccordionDetails>
              <Options
                {...{
                  length,
                  includeUppercase,
                  includeLowercase,
                  includeSymbols,
                  setLength,
                  includeNumber,
                  setIncludeNumber,
                  setIncludeUppercase,
                  setIncludeLowercase,
                  setIncludeSymbols,
                }}
              />
            </AccordionDetails>
          </Accordion>
          <Button
            variant="contained"
            onClick={() => setPassword(generatePassword())}
            startIcon={<LoopIcon />}
          >
            Generate
          </Button>
        </Card>
      </Container>
    </>
  );
}

export default Form;
