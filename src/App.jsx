import {useState} from "react";
import "./App.css";
import {Tab, Tabs, TabList, TabPanel} from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import './ReactTabsOverrides.css'
import StagingTab from "./storage/StagingTab.jsx";
import InspectionTab from "./storage/InspectionTab.jsx";
import ScanTab from "./storage/ScanTab.jsx";

import StorageIcon from '@mui/icons-material/Storage';
import ComputeIcon from '@mui/icons-material/Computer'; // Assuming this icon for Compute
import MemoryIcon from '@mui/icons-material/Memory';
import NetworkCheckIcon from '@mui/icons-material/NetworkCheck'; // Assuming this icon for Network
import SearchIcon from '@mui/icons-material/Search';
import FindInPageIcon from '@mui/icons-material/FindInPage';
import LayersIcon from '@mui/icons-material/Layers';
import Co2Icon from '@mui/icons-material/Co2';


function App() {
    const [selectedRows, setSelectedRows] = useState([]);

    return (<Tabs forceRenderTabPanel defaultIndex={0}>
        <TabList>
            <TabList>
                <Tab>
                    <div style={{ display: 'flex', alignItems: 'center' }}>
                        <StorageIcon style={{ marginRight: '8px' }} />Storage
                    </div>
                </Tab>
                <Tab>
                    <div style={{ display: 'flex', alignItems: 'center' }}>
                        <ComputeIcon style={{ marginRight: '8px' }} />Compute
                    </div>
                </Tab>
                <Tab>
                    <div style={{ display: 'flex', alignItems: 'center' }}>
                        <MemoryIcon style={{ marginRight: '8px' }} />Memory
                    </div>
                </Tab>
                <Tab>
                    <div style={{ display: 'flex', alignItems: 'center' }}>
                        <NetworkCheckIcon style={{ marginRight: '8px' }} />Network
                    </div>
                </Tab>

                <Tab>
                    <div style={{display: 'flex', alignItems: 'center' }}>
                        <Co2Icon style={{ marginRight: '8px' }} />Carbon Aware
                    </div>
                </Tab>
            </TabList>
            <TabPanel>
                {/* Your TabPanel content */}
            </TabPanel>
        </TabList>
        <TabPanel>
            <Tabs forceRenderTabPanel>
                <TabList>
                    <Tab>
                        <div style={{display: 'flex', alignItems: 'center' }}>
                            <SearchIcon style={{ marginRight: '8px' }} />Scan
                        </div>
                    </Tab>
                    <Tab>
                        <div style={{display: 'flex', alignItems: 'center' }}>
                            <FindInPageIcon style={{ marginRight: '8px' }} />Inspections
                        </div>
                    </Tab>
                    <Tab>
                        <div style={{display: 'flex', alignItems: 'center' }}>
                            <LayersIcon style={{ marginRight: '8px' }} />Staging
                        </div>
                    </Tab>
                </TabList>
                <TabPanel>
                    <ScanTab></ScanTab>
                </TabPanel>
                <TabPanel>
                    <InspectionTab setSelectedRows={setSelectedRows}></InspectionTab>
                </TabPanel>
                <TabPanel>
                    <StagingTab selectedRows={selectedRows}></StagingTab>
                </TabPanel>
            </Tabs>
        </TabPanel>
        <TabPanel>
            <Tabs forceRenderTabPanel>
                <TabList>
                    <Tab>Info</Tab>
                </TabList>
                <TabPanel>
                </TabPanel>
            </Tabs>
        </TabPanel>
    </Tabs>);
}

export default App;