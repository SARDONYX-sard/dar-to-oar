"use client";

import * as React from "react";
import BottomNavigation from "@mui/material/BottomNavigation";
import BottomNavigationAction from "@mui/material/BottomNavigationAction";
import TransformIcon from "@mui/icons-material/Transform";
import SettingsIcon from "@mui/icons-material/Settings";
import { styled } from "@mui/material";

const Nav = styled(BottomNavigation)(() => {
  return {
    position: "fixed",
    bottom: 0,
    width: "100%",
    ".Mui-selected": {
      color: "#99e4ee",
    },
  };
});

export default function MenuNavigation() {
  const [value, setValue] = React.useState(0);

  return (
    <Nav
      showLabels
      value={value}
      onChange={(event, newValue) => {
        setValue(newValue);
      }}
    >
      <BottomNavigationAction label="Convert" icon={<TransformIcon />} />
      <BottomNavigationAction label="Settings" icon={<SettingsIcon />} />
    </Nav>
  );
}
