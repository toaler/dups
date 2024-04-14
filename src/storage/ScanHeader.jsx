import React, {useState} from 'react';
import styled from "styled-components"
import DirectionsRunIcon from '@mui/icons-material/DirectionsRun';

const formatElapsedTime = (elapsedTime) => {
    // Convert elapsed time to hours, minutes, and milliseconds
    const hours = Math.floor(elapsedTime / 3600000); // Total hours
    const minutes = Math.floor((elapsedTime % 3600000) / 60000); // Remaining minutes
    const seconds = Math.floor((elapsedTime % 60000) / 1000); // Convert remainder to seconds
    const milliseconds = elapsedTime % 1000; // Milliseconds are the remainder of elapsed time divided by 1000

    // Format milliseconds to ensure it's always displayed as a three-digit number
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${milliseconds.toString().padStart(3, '0')}`;
};

function ScanHeader({status, elapsedTime, resources, directories, files, size}) {

    return <ScanHeaderContainer>
        {console.log(elapsedTime)}
        <ScanHeaderLeft>
            <div className="flex-container">
                <div className="flex-row">
                    <div className="flex-item">Status</div>
                    <div className="flex-item">{status}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Elapsed</div>
                    <div className="flex-item">{formatElapsedTime(elapsedTime)}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Resources</div>
                    <div className="flex-item">{Number(resources).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Directories</div>
                    <div className="flex-item">{Number(directories).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Files</div>
                    <div className="flex-item">{Number(files).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Size</div>
                    <div className="flex-item">{Number(size).toLocaleString()}</div>
                </div>
            </div>
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