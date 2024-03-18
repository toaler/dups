import {useState} from "react";
import "./App.css";
import {Tab, Tabs, TabList, TabPanel} from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import StagingTab from "./storage/StagingTab.jsx";
import InspectionTab from "./storage/InspectionTab.jsx";
import ScanTab from "./storage/ScanTab.jsx";

function App() {
    const [selectedRows, setSelectedRows] = useState([]);

    return (<Tabs forceRenderTabPanel defaultIndex={0}>
        <TabList>
            <Tab>Storage</Tab>
            <Tab>Compute</Tab>
            <Tab>Memory</Tab>
            <Tab>Network</Tab>
        </TabList>
        <TabPanel>
            <Tabs forceRenderTabPanel>
                <TabList>
                    <Tab>Scan</Tab>
                    <Tab>Inspections</Tab>
                    <Tab>Staging</Tab>
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
                    <Tab>Foo</Tab>
                </TabList>
                <TabPanel>
                    <p>bar</p>
                    <img
                        src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png"
                        alt="Philip J. Fry"/>
                </TabPanel>
            </Tabs>
        </TabPanel>
    </Tabs>);
}

export default App;