import React, {useState} from 'react';
import styled from "styled-components"
import DirectionsRunIcon from '@mui/icons-material/DirectionsRun';

function ScanHeader() {

    const [resources, setResources] = useState(0);
    const [directories, setDirectories] = useState(0);
    const [files, setFiles] = useState(0);
    const [size, setSize] = useState(0);

    return <ScanHeaderContainer>
        <ScanHeaderLeft>
            <h1>{Number(resources).toLocaleString()}</h1>
            <h1>Directories</h1>
            <h1>{Number(directories).toLocaleString()}</h1>
            <h1>Files</h1>
            <h1>{Number(files).toLocaleString()}</h1>
            <h1>Size</h1>
            <h1>{Number(size).toLocaleString()}</h1>
        </ScanHeaderLeft>
    </ScanHeaderContainer>
}

export default ScanHeader

const ScanHeaderContainer = styled.div`
    display: flex;
    font-family: San Francisco;
    font-size: 12px;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
`;

const ScanHeaderLeft = styled.div`
    display: flex;
    align-items: center; // Adjust vertical alignment as needed
    justify-content: space-between; // Spread out the children to use available space
    flex-wrap: wrap; // Allow items to wrap to a new line if needed
    gap: 20px; // Creates space between items
`;


const ScanRunIcon = styled(DirectionsRunIcon)`
    display: flex;
    font-size: 4rem !important; // Adjust the size as needed
    margin-left: auto;
    margin-right: 30px;
    color: inherit; // Or specify a color
`;