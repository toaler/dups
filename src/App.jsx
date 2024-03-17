import {useState} from "react";
import "./App.css";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import StorageStagingTab from "./StorageStagingTab.jsx";
import StorageInspectionTab from "./StorageInspectionTab.jsx";
import StorageScanTab from "./StorageScanTab.jsx";

function App() {
  const [selectedRows, setSelectedRows] = useState([]);

  return (
 <Tabs forceRenderTabPanel defaultIndex={0}>
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
          <StorageScanTab></StorageScanTab>
        </TabPanel>
        <TabPanel>
          <StorageInspectionTab setSelectedRows={setSelectedRows}></StorageInspectionTab>
        </TabPanel>
        <TabPanel>
          <StorageStagingTab selectedRows={selectedRows}></StorageStagingTab>
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
         <img src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png"
              alt="Philip J. Fry"/>
       </TabPanel>
      </Tabs>
    </TabPanel>
  </Tabs>
  );
}

export default App;