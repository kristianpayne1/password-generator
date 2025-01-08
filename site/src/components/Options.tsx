import FormControl from "@mui/material/FormControl";
import {
  FormLabel,
  Grid2 as Grid,
  Input,
  Slider,
  Box,
  Switch,
} from "@mui/material";

type OptionsType = {
  length: number;
  includeNumber: boolean;
  includeUppercase: boolean;
  includeLowercase: boolean;
  includeSymbols: boolean;
  setLength: (length: number) => void;
  setIncludeNumber: (includeNumber: boolean) => void;
  setIncludeUppercase: (includeUppercase: boolean) => void;
  setIncludeLowercase: (includeLowercase: boolean) => void;
  setIncludeSymbols: (includeSymbols: boolean) => void;
};

const FormControlSX = {
  width: "100%",
  display: "flex",
  flexDirection: "row",
  alignItems: "center",
  justifyContent: "space-between",
};

export function Options({
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
}: OptionsType) {
  return (
    <Box>
      <FormControl sx={{ width: "100%" }}>
        <FormLabel htmlFor="password-length">Password Length</FormLabel>
        <Grid container spacing={1} sx={{ alignItems: "center" }}>
          <Grid size="grow">
            <Slider
              value={length}
              onChange={(e: Event) => {
                const element = e.target as HTMLInputElement;
                setLength(+element.value);
              }}
              id="password-length"
              min={5}
              max={127}
              valueLabelDisplay="auto"
            />
          </Grid>
          <Grid>
            <Input
              value={length}
              size="small"
              onChange={(e) => setLength(+e.target?.value)}
              inputProps={{
                step: 0,
                min: 5,
                max: 127,
                type: "number",
              }}
            />
          </Grid>
        </Grid>
      </FormControl>
      <FormControl sx={FormControlSX}>
        <FormLabel htmlFor="include-numbers">Include Numbers</FormLabel>
        <Switch
          id="include-numbers"
          checked={includeNumber}
          onChange={(_, value) => setIncludeNumber(value)}
        />
      </FormControl>
      <FormControl sx={FormControlSX}>
        <FormLabel htmlFor="include-uppercase">Include Uppercase</FormLabel>
        <Switch
          checked={includeUppercase}
          onChange={(_, value) => setIncludeUppercase(value)}
        />
      </FormControl>
      <FormControl sx={FormControlSX}>
        <FormLabel htmlFor="include-lowercase">Include Lowercase</FormLabel>
        <Switch
          checked={includeLowercase}
          onChange={(_, value) => setIncludeLowercase(value)}
        />
      </FormControl>
      <FormControl sx={FormControlSX}>
        <FormLabel htmlFor="include-symbols">Include Symbols</FormLabel>
        <Switch
          checked={includeSymbols}
          onChange={(_, value) => setIncludeSymbols(value)}
        />
      </FormControl>
    </Box>
  );
}
