"use client";

import { useEffect, useState } from "react";
import BottomNavigation from "@mui/material/BottomNavigation";
import BottomNavigationAction from "@mui/material/BottomNavigationAction";
import TransformIcon from "@mui/icons-material/Transform";
import SettingsIcon from "@mui/icons-material/Settings";
import { styled } from "@mui/material";
import { useRouter } from "next/navigation";

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
  const router = useRouter();
  const [value, setValue] = useState(0);

  useEffect(() => {
    const location = window.location.pathname;
    if (location === "/") {
      setValue(0);
    } else if (location === "/settings") {
      setValue(1);
    }
  }, [setValue]);

  return (
    <Nav
      showLabels
      value={value}
      onChange={(_event, newValue) => {
        setValue(newValue);
      }}
    >
      <BottomNavigationAction
        label="Convert"
        icon={<TransformIcon />}
        onClick={() => router.push("/")}
      />
      <BottomNavigationAction
        label="Settings"
        icon={<SettingsIcon />}
        onClick={() => router.push("/settings")}
      />
    </Nav>
  );
}
